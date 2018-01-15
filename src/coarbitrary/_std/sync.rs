//! CoArbitrary for ::std::sync.
use coarbitrary::*;

use std::sync::*;
use std::sync::atomic::*;
use std::sync::mpsc::*;

// TODO: We could possibly support Mutex and RwLock.

// TODO: Consider what Ordering to use for atomic::Atomic<X> types.
// or simply skip those types.

// Barrier.wait() has side-effects => skip.
// Same for Condvar.
// Same for Once.

delegate_deref!([A: CoArbitrary + ?Sized] Arc<A>);
coarbitrary!([A: CoArbitrary + ?Sized] Weak<A>;
    self, var => var.nest(&self.upgrade()));

coarbitrary!(BarrierWaitResult; self, var => var.nest(&self.is_leader()));
coarbitrary!(WaitTimeoutResult; self, var => var.nest(&self.timed_out()));
delegate_deref!(['a, T: ?Sized + CoArbitrary] MutexGuard<'a, T>);
delegate_deref!(['a, T: ?Sized + CoArbitrary] RwLockReadGuard<'a, T>);
delegate_deref!(['a, T: ?Sized + CoArbitrary] RwLockWriteGuard<'a, T>);
coarbitrary!([T: CoArbitrary] PoisonError<T>;
    self, var => var.nest(self.get_ref()));
coarbitrary!([T: CoArbitrary] TryLockError<T>; self, var => match *self {
    TryLockError::Poisoned(ref a) => var.variant(0).nest(a),
    TryLockError::WouldBlock => var.variant(1),
});

#[cfg(feature = "unstable")]
coarbitrary!(OnceState; self, var => var.nest(&self.poisoned()));
coarbitrary!(Ordering; self, var => match *self {
    Ordering::Relaxed => var.variant(0),
    Ordering::Release => var.variant(1),
    Ordering::Acquire => var.variant(2),
    Ordering::AcqRel => var.variant(3),
    Ordering::SeqCst => var.variant(4),
    _ => var.variant(5),
});
coarbitrary!(RecvTimeoutError; self, var => match *self {
    RecvTimeoutError::Timeout => var.variant(0),
    RecvTimeoutError::Disconnected => var.variant(1),
});
coarbitrary!(TryRecvError; self, var => match *self {
    TryRecvError::Empty => var.variant(0),
    TryRecvError::Disconnected => var.variant(1),
});
coarbitrary!([T: CoArbitrary] TrySendError<T>; self, var => match *self {
    TrySendError::Full(ref a) => var.variant(0).nest(a),
    TrySendError::Disconnected(ref a) => var.variant(1).nest(a),
});

#[cfg(feature = "unstable")]
coarbitrary!(['rx, T: Send + CoArbitrary] Handle<'rx, T>;
    self, var => var.nest(&self.id()));

coarbitrary_unit!(RecvError);

coarbitrary!([T: CoArbitrary] SendError<T>; self, var => var.nest(&self.0));