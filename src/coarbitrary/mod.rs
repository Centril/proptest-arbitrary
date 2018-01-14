//! Defines the `CoArbitrary` trait and implementations for it.

mod traits;
pub use self::traits::*;

#[macro_use]
mod macros;

mod primitive;

mod _std;

/*

//==============================================================================
// Strategy for closures:
//==============================================================================.

mod closure;
pub use self::closure::*;

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
            input.coarbitrary(Perturbable::new(&mut rng));
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

*/