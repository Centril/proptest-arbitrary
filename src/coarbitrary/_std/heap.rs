use coarbitrary::*;

use std::heap::*;

coarbitrary_unit!(CannotReallocInPlace, Heap, System);

impl CoArbitrary for Excess {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&(self.0 as usize))
           .nest(&self.1);
    }
}

impl CoArbitrary for Layout {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.size())
           .nest(&self.align());
    }
}