//! CoArbitrary for ::std::boxed.

use coarbitrary::*;

impl<A: CoArbitrary + ?Sized> CoArbitrary for Box<A> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&**self);
    }
}