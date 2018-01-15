use coarbitrary::*;

use std::thread::*;

delegate_hash!([] ThreadId);

coarbitrary!(Thread; self, var => var.nest(&self.id()).nest(&self.name()));

coarbitrary!([T: CoArbitrary] JoinHandle<T>;
    self, var => var.nest(self.thread()));

#[cfg(feature = "unstable")]
coarbitrary!(LocalKeyState; self, var => match *self {
    LocalKeyState::Uninitialized => var.variant(0),
    LocalKeyState::Valid => var.variant(1),
    LocalKeyState::Destroyed => var.variant(2),
});

#[cfg(feature = "unstable")]
coarbitrary_unit!(AccessError);