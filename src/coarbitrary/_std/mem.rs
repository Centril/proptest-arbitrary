use coarbitrary::*;

use std::mem::{Discriminant, ManuallyDrop};

delegate_hash!([T] Discriminant<T>);

impl<T: CoArbitrary> CoArbitrary for ManuallyDrop<T> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&**self);
    }
}