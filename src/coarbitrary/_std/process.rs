use coarbitrary::*;

use std::process::*;

impl CoArbitrary for Child {
    fn coarbitrary(&self, mut var: Perturbable) {
        // This OK -  .id()  is a pure function even tho everything else in
        // Child is impure.
        var.nest(&self.id());
    }
}

impl CoArbitrary for ExitStatus {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.code());
    }
}

impl CoArbitrary for Output {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.status)
           .nest(&self.stdout)
           .nest(&self.stderr);
    }
}