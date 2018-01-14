//! CoArbitrary for ::std::rc.

use coarbitrary::*;

use std::rc::*;

impl<A: CoArbitrary + ?Sized> CoArbitrary for Rc<A> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&**self);
    }
}

impl<A: CoArbitrary + ?Sized> CoArbitrary for Weak<A> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.upgrade());
    }
}