//! Defines CoArbitrary and a Strategy for generating a function.
//!
//! This is highly experimental at this stage.

use proptest::prelude::XorShiftRng;

//==============================================================================
// Perturbing helpers:
//==============================================================================.

/// Perturbs the RNG with an integer seed.
pub fn variant(rng: &mut XorShiftRng, seed: usize) {
    unimplemented!()
}

/// Helper for convenient defining of `CoArbitrary`.
pub struct Varianter<'rng> {
    rng: &'rng mut XorShiftRng
}

impl<'rng> Varianter<'rng> {
    /// Constructs a new helper given the underlying RNG.
    pub fn new(rng: &'rng mut XorShiftRng) -> Self {
        Self { rng }
    }

    /// Returns the underlying RNG.
    pub fn rng(self) -> &'rng mut XorShiftRng {
        self.rng
    }

    /// Perturbs the underlying RNG with an integer seed.
    pub fn variant(&mut self, seed: usize) -> &mut Self {
        variant(self.rng, seed);
        self
    }

    /// Perturbs the underlying RNG using the
    /// `CoArbitrary` impl of the given object.
    pub fn nest<C: CoArbitrary + ?Sized>(&mut self, nested: &C) -> &mut Self {
        nested.coarbitrary(Self::new(self.rng));
        self
    }
}

//==============================================================================
// CoArbitrary:
//==============================================================================.

/// `CoArbitrary` defines a method for perturbing an RNG
/// based on the value of `&self`.
pub trait CoArbitrary {
    /// Perturbs the given underlying RNG according to
    /// the structure of the given `&self` value.
    fn coarbitrary(&self, var: Varianter);
}

impl<'a, A: CoArbitrary> CoArbitrary for &'a A {
    fn coarbitrary(&self, mut var: Varianter) {
        var.nest(self);
    }
}

impl<'a, A: CoArbitrary> CoArbitrary for &'a mut A {
    fn coarbitrary(&self, mut var: Varianter) {
        var.nest(self);
    }
}

impl<A: CoArbitrary> CoArbitrary for Box<A> {
    fn coarbitrary(&self, mut var: Varianter) {
        var.nest(self.as_ref());
    }
}

impl CoArbitrary for bool {
    fn coarbitrary(&self, mut var: Varianter) {
        var.variant(if *self { 1 } else { 1 });
    }
}

impl<A: CoArbitrary> CoArbitrary for Option<A> {
    fn coarbitrary(&self, mut var: Varianter) {
        match *self {
            None => var.variant(0),
            Some(ref x) => var.variant(1).nest(x),
        };
    }
}

impl<A: CoArbitrary> CoArbitrary for [A] {
    fn coarbitrary(&self, mut var: Varianter) {
        for x in self {
            var.variant(1).nest(x);
        }
        var.variant(0);
    }
}

impl<A: CoArbitrary> CoArbitrary for Vec<A> {
    fn coarbitrary(&self, mut var: Varianter) {
        var.nest(&*self);
    }
}

macro_rules! tuple_coarbitrary {
    ( $($ty: ident)* ) => {
        impl<$($ty : CoArbitrary),*> CoArbitrary for ($($ty,)*) {
            #[allow(unused_mut)]
            #[allow(non_snake_case)]
            fn coarbitrary(&self, mut _var: Varianter) {
                let &($(ref $ty,)*) = self;
                $(_var.nest($ty);)*
            }
        }
    };
}

tuple_coarbitrary!();
tuple_coarbitrary!(T0);
tuple_coarbitrary!(T0 T1);
tuple_coarbitrary!(T0 T1 T2);
tuple_coarbitrary!(T0 T1 T2 T3);
tuple_coarbitrary!(T0 T1 T2 T3 T4);
tuple_coarbitrary!(T0 T1 T2 T3 T4 T5);
tuple_coarbitrary!(T0 T1 T2 T3 T4 T5 T6);
tuple_coarbitrary!(T0 T1 T2 T3 T4 T5 T6 T7);
tuple_coarbitrary!(T0 T1 T2 T3 T4 T5 T6 T7 T8);
tuple_coarbitrary!(T0 T1 T2 T3 T4 T5 T6 T7 T8 T9);

//==============================================================================
// Tower of Fn-like traits:
//==============================================================================.

/// Same semantics as `FnOnce`.
pub trait ApplyOnce<Args> {
    /// The output type of this closure.
    type Output;

    /// Calls the closure given args and returns a value of the output type.
    /// This consumes the closure.
    fn apply_once(self, args: Args) -> Self::Output;
}

/// Same semantics as `FnMut`.
pub trait ApplyMut<Args>: ApplyOnce<Args> {
    /// Calls the closure given args and returns a value of the output type.
    /// This does not consume the closure and allows
    /// the state of the closure to be modified.
    fn apply_mut(&mut self, args: Args) -> Self::Output;
}

/// Same semantics as `Fn`.
pub trait Apply<Args>: ApplyMut<Args> {
    /// Calls the closure given args and returns a value of the output type.
    /// This does not consume the closure and does not allow mutation of
    /// the closure's state.
    fn apply(&self, input: Args) -> Self::Output;

    /// Converts the closure into a boxed `Fn` closure.
    fn to_closure(self) -> Box<Fn(Args) -> Self::Output>
    where
        Args: 'static,
        Self::Output: 'static,
        Self: Sized + 'static,
    {
        Box::new(move |input| self.apply(input))
    }
}

//==============================================================================
// Strategy for closures:
//==============================================================================.

use proptest::test_runner::TestRunner;
use proptest::strategy::{Strategy, ValueTree, ValueFor};

use std::marker::PhantomData;
use std::fmt;
use std::sync::Arc;

struct GenClosure<A, B> {
    input_t: PhantomData<A>,
    bstrat: B, // <-- Arc here too?
    runner: Arc<TestRunner>,
}

impl<A, B> GenClosure<A, B> {
    /// Constructs a new closure from the given strategy and runner.
    fn new(bstrat: B, runner: TestRunner) -> Self {
        Self {
            input_t: PhantomData,
            bstrat,
            runner: Arc::new(runner),
        }
    }
}

impl<A, B: Clone> Clone for GenClosure<A, B> {
    fn clone(&self) -> Self {
        Self {
            input_t: PhantomData,
            bstrat: self.bstrat.clone(),
            runner: self.runner.clone()
        }
    }
}

impl<A, B> fmt::Debug for GenClosure<A, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("GenClosure")
    }
}

impl<A: CoArbitrary, B: Strategy> Apply<A> for GenClosure<A, B> {
    fn apply(&self, input: A) -> ValueFor<B> {
        // Deep clone of runner:
        let mut runner = (&*self.runner).clone();

        // Perturb rng by input:
        {
            let mut rng = runner.rng();
            input.coarbitrary(Varianter::new(&mut rng));
        }

        // Generate new value:
        let new_val = self.bstrat.new_value(&mut runner);

        // Since we have to return a `ValueFor<B>`, we have no choice
        // but to panic if the strategy didn't produce a tree.
        let val = new_val.unwrap();

        val.current()
    }
}

impl<A: CoArbitrary, B: Strategy> ApplyMut<A> for GenClosure<A, B> {
    fn apply_mut(&mut self, input: A) -> Self::Output {
        self.apply(input)
    }
}

impl<A: CoArbitrary, B: Strategy> ApplyOnce<A> for GenClosure<A, B> {
    type Output = ValueFor<B>;

    fn apply_once(self, input: A) -> Self::Output {
        self.apply(input)
    }
}

struct ClosureStrategy<A, B> {
    bstrat: B,
    __phantom: PhantomData<A>,
}

impl<A, B> fmt::Debug for ClosureStrategy<A, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ClosureStrategy")
    }
}

impl<A: CoArbitrary, B: Strategy + Clone> Strategy for ClosureStrategy<A, B> {
    type Value = GenClosure<A, B>;

    fn new_value(&self, runner: &mut TestRunner)
        -> Result<Self::Value, String>
    {
        Ok(GenClosure::new(self.bstrat.clone(), runner.clone()))
    }
}

impl<A: CoArbitrary, B: Strategy + Clone> ValueTree for GenClosure<A, B> {
    type Value = GenClosure<A, B>;

    fn current(&self) -> Self::Value {
        (*self).clone()
    }

    fn simplify(&mut self) -> bool { false }
    fn complicate(&mut self) -> bool { false }
}