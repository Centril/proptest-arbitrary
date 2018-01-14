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

impl<A: CoArbitrary + ?Sized> CoArbitrary for Arc<A> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&**self);
    }
}

impl<A: CoArbitrary + ?Sized> CoArbitrary for Weak<A> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.upgrade());
    }
}

impl CoArbitrary for BarrierWaitResult {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.is_leader());
    }
}

impl CoArbitrary for WaitTimeoutResult {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.timed_out());
    }
}

impl<'a, T: ?Sized + CoArbitrary> CoArbitrary for MutexGuard<'a, T> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&**self);
    }
}

impl<'a, T: ?Sized + CoArbitrary> CoArbitrary for RwLockReadGuard<'a, T> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&**self);
    }
}

impl<'a, T: ?Sized + CoArbitrary> CoArbitrary for RwLockWriteGuard<'a, T> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&**self);
    }
}

impl<T: CoArbitrary> CoArbitrary for PoisonError<T> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(self.get_ref());
    }
}

impl<T: CoArbitrary> CoArbitrary for TryLockError<T> {
    fn coarbitrary(&self, mut var: Perturbable) {
        match *self {
            TryLockError::Poisoned(ref a) => var.variant(0).nest(a),
            TryLockError::WouldBlock => var.variant(1),
        };
    }
}

#[cfg(feature = "unstable")]
impl CoArbitrary for OnceState {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.poisoned());
    }
}

impl CoArbitrary for Ordering {
    fn coarbitrary(&self, mut var: Perturbable) {
        match *self {
            Ordering::Relaxed => var.variant(0),
            Ordering::Release => var.variant(1),
            Ordering::Acquire => var.variant(2),
            Ordering::AcqRel => var.variant(3),
            Ordering::SeqCst => var.variant(4),
            _ => var.variant(5),
        };
    }
}

impl CoArbitrary for RecvTimeoutError {
    fn coarbitrary(&self, mut var: Perturbable) {
        match *self {
            RecvTimeoutError::Timeout => var.variant(0),
            RecvTimeoutError::Disconnected => var.variant(1),
        };
    }
}

impl CoArbitrary for TryRecvError {
    fn coarbitrary(&self, mut var: Perturbable) {
        match *self {
            TryRecvError::Empty => var.variant(0),
            TryRecvError::Disconnected => var.variant(1),
        };
    }
}

impl<T: CoArbitrary> CoArbitrary for TrySendError<T> {
    fn coarbitrary(&self, mut var: Perturbable) {
        match *self {
            TrySendError::Full(ref a) => var.variant(0).nest(a),
            TrySendError::Disconnected(ref a) => var.variant(1).nest(a),
        };
    }
}

#[cfg(feature = "unstable")]
impl<'rx, T: Send + CoArbitrary> CoArbitrary for Handle<'rx, T> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.id());
    }
}

coarbitrary_unit!(RecvError);

impl<T: CoArbitrary> CoArbitrary for SendError<T> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.0);
    }
}