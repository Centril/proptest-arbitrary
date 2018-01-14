//! CoArbitrary for ::std::result.

use coarbitrary::*;

use std::result::*;

impl<A: CoArbitrary, B: CoArbitrary> CoArbitrary for Result<A, B> {
    fn coarbitrary(&self, mut var: Perturbable) {
        match *self {
            Err(ref err) => var.variant(0).nest(err),
            Ok(ref ok) => var.variant(1).nest(ok),
        };
    }
}

impl<A: CoArbitrary + Clone> CoArbitrary for IntoIter<A> {
    fn coarbitrary(&self, var: Perturbable) {
        coarbitrary_iter(self.clone(), var);
    }
}

impl<'a, A: CoArbitrary> CoArbitrary for Iter<'a, A> {
    fn coarbitrary(&self, var: Perturbable) {
        coarbitrary_iter(self.clone(), var);
    }
}