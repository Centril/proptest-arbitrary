use coarbitrary::*;

use std::thread::*;

delegate_hash!([] ThreadId);

impl CoArbitrary for Thread {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.id()).nest(&self.name());
    }
}

impl<T: CoArbitrary> CoArbitrary for JoinHandle<T> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(self.thread());
    }
}

#[cfg(feature = "unstable")]
impl CoArbitrary for LocalKeyState {
    fn coarbitrary(&self, mut var: Perturbable) {
        match *self {
            LocalKeyState::Uninitialized => var.variant(0),
            LocalKeyState::Valid => var.variant(1),
            LocalKeyState::Destroyed => var.variant(2),
        };
    }
}

#[cfg(feature = "unstable")]
coarbitrary_unit!(AccessError);