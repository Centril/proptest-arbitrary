//! CoArbitrary for ::std::panic.

use coarbitrary::*;

use std::panic::*;

impl<A: CoArbitrary> CoArbitrary for AssertUnwindSafe<A> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&**self);
    }
}

impl<'a> CoArbitrary for Location<'a> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.file())
           .nest(&self.line())
           .nest(&self.column());
    }
}