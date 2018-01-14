use coarbitrary::*;

use std::time::*;

delegate_hash!([] Duration);
delegate_hash!([] Instant);
delegate_hash!([] SystemTime);

impl CoArbitrary for SystemTimeError {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.duration());
    }
}