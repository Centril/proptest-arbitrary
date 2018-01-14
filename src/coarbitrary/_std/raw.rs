use coarbitrary::*;

use std::raw::*;

impl CoArbitrary for TraitObject {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&(self.data as usize))
           .nest(&(self.vtable as usize));
    }
}
