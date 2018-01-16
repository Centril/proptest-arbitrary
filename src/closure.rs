//! Provides `ClosureStrategy<A, B>` which generates `RandomClosure<A, B>`
//! which is morally an `Fn(A) -> B`. You can convert `RandomClosure<A, B>`
//! into `Box<Fn(A) -> B>`.

use coarbitrary::*;

use proptest::test_runner::TestRunner;
use proptest::strategy::{Strategy, ValueTree, NewTree, ValueFor};

use std::marker::PhantomData;
use std::fmt;
use std::sync::Arc;

//==============================================================================
// Fn-like traits in stable Rust:
//==============================================================================.

/// Same semantics as `FnOnce`.
///
/// This is used at the moment because the `Fn`-family traits
/// are unstable and can not be used in stable Rust.
pub trait ApplyOnce<Args> {
    /// The output type of this closure.
    type Output;

    /// Calls the closure given args and returns a value of the output type.
    /// This consumes the closure.
    fn apply_once(self, args: Args) -> Self::Output;
}

/// Same semantics as `FnMut`.
///
/// This is used at the moment because the `Fn`-family traits
/// are unstable and can not be used in stable Rust.
pub trait ApplyMut<Args>: ApplyOnce<Args> {
    /// Calls the closure given args and returns a value of the output type.
    /// This does not consume the closure and allows
    /// the state of the closure to be modified.
    fn apply_mut(&mut self, args: Args) -> Self::Output;
}

/// Same semantics as `Fn`.
///
/// This is used at the moment because the `Fn`-family traits
/// are unstable and can not be used in stable Rust.
pub trait Apply<Args>: ApplyMut<Args> {
    /// Calls the closure given args and returns a value of the output type.
    /// This does not consume the closure and does not allow mutation of
    /// the closure's state.
    fn apply(&self, input: Args) -> Self::Output;

    /// Converts the closure into a boxed `Fn` closure.
    fn to_boxed_fn(self) -> Box<Fn(Args) -> Self::Output>
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

/// Morally, this encodes a function (closure) of type `A -> B`.
/// You can think of it as a randomly generated `Fn(A) -> B`.
pub struct RandomClosure<A, B> {
    input: PhantomData<A>,
    /// The strategy to generate outputs from:
    output: B,
    /// The runner we pass to `output`.
    runner: Arc<TestRunner>,
}

impl<A, B> RandomClosure<A, B> {
    /// Constructs a new function (closure) from the given strategy for
    /// output types and a runner.
    fn new(output: B, runner: TestRunner) -> Self {
        Self {
            input: PhantomData,
            output,
            runner: Arc::new(runner),
        }
    }
}

impl<A: CoArbitrary, B: Strategy> Apply<A> for RandomClosure<A, B> {
    fn apply(&self, input: A) -> ValueFor<B> {
        // Deep clone of runner so that we maintain purity!
        let mut runner = (&*self.runner).clone();

        // Perturb PRNG by input:
        {
            input.coarbitrary(Perturbable::new(runner.rng()));
        }

        // Generate new value:
        let output = self.output.new_value(&mut runner);

        // Since we have to return a `ValueFor<B>`, we have no choice
        // but to panic if the strategy didn't produce a tree.
        output.unwrap().current()
    }
}

impl<A: CoArbitrary, B: Strategy> ApplyMut<A> for RandomClosure<A, B> {
    fn apply_mut(&mut self, input: A) -> Self::Output {
        self.apply(input)
    }
}

impl<A: CoArbitrary, B: Strategy> ApplyOnce<A> for RandomClosure<A, B> {
    type Output = ValueFor<B>;

    fn apply_once(self, input: A) -> Self::Output {
        self.apply(input)
    }
}

impl<A, B: Clone> Clone for RandomClosure<A, B> {
    fn clone(&self) -> Self {
        Self {
            input: PhantomData,
            output: self.output.clone(),
            runner: self.runner.clone()
        }
    }
}

impl<A, B> fmt::Debug for RandomClosure<A, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("RandomClosure")
    }
}

/// A `Strategy` that generates random functions which are morally
/// of type `Fn(A) -> B` based on an output `Strategy` for `B` and
/// the ability of the input type `A` to perturb the PRNG that is
/// used to produce concerete `B`s.
pub struct ClosureStrategy<A, B> {
    input: PhantomData<A>,
    /// The strategy to generate outputs from:
    output: B,
}

impl<A: CoArbitrary, B: Strategy + Clone> Strategy for ClosureStrategy<A, B> {
    type Value = RandomClosure<A, B>;

    fn new_value(&self, runner: &mut TestRunner) -> NewTree<Self> {
        Ok(RandomClosure::new(self.output.clone(), runner.clone()))
    }
}

impl<A: CoArbitrary, B: Strategy + Clone> ValueTree for RandomClosure<A, B> {
    type Value = RandomClosure<A, B>;

    fn current(&self) -> Self::Value {
        (*self).clone()
    }

    fn simplify(&mut self) -> bool { false }
    fn complicate(&mut self) -> bool { false }
}

impl<A, B> fmt::Debug for ClosureStrategy<A, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ClosureStrategy")
    }
}