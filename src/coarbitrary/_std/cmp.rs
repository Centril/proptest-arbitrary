//! CoArbitrary for ::std::collections.

use coarbitrary::*;

use std::cmp::*;

impl<A: CoArbitrary> CoArbitrary for Reverse<A> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.0);
    }
}

impl CoArbitrary for Ordering {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.variant(match *self {
            Ordering::Greater => 0,
            Ordering::Equal => 1,
            Ordering::Less => 2,
        });
    }
}