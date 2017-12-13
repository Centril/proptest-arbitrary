use super::*;
use std::sync::*;
use std::sync::atomic::*;
use std::sync::mpsc::*;
use std::thread;
use std::time::Duration;

// OnceState can not escape Once::call_once_force.
// PoisonError depends implicitly on the lifetime on MutexGuard, etc.
// This transitively applies to TryLockError.

wrap_from!(Arc);

#[cfg(not(MIN_VER_1_24_0))]
wrap_ctor!(Mutex);
#[cfg(MIN_VER_1_24_0)]
wrap_from!(Mutex);

#[cfg(not(MIN_VER_1_24_0))]
wrap_ctor!(RwLock);
#[cfg(MIN_VER_1_24_0)]
wrap_from!(RwLock);

arbitrary!(Barrier, SMapped<'a, u16, Self>;  // usize would be extreme!
    static_map(any::<u16>(), |n| Barrier::new(n as usize))
);

arbitrary!(BarrierWaitResult,
    TupleUnion<(W<GenStrategy<Self>>, W<GenStrategy<Self>>)>;
    prop_oneof![GenStrategy::new(bwr_true), GenStrategy::new(bwr_false)]
);

generator!(
    Condvar, default;
    Once, Once::new
);

arbitrary!(WaitTimeoutResult, TupleUnion<(W<Just<Self>>, W<Just<Self>>)>;
    prop_oneof![Just(wtr_true()), Just(wtr_false())]
);

fn bwr_true() -> BarrierWaitResult {
    Barrier::new(1).wait()
}

fn bwr_false() -> BarrierWaitResult {
    let barrier = Arc::new(Barrier::new(2));
    let b2 = barrier.clone();
    let jh = thread::spawn(move|| { b2.wait() });
    let bwr1 = barrier.wait();
    let bwr2 = jh.join().unwrap();
    if bwr1.is_leader() { bwr2 } else { bwr1 }
}

fn wtr_false() -> WaitTimeoutResult {
    let cvar = Arc::new(Condvar::new());
    let cvar2 = cvar.clone();
    thread::spawn(move|| { cvar2.notify_one(); });
    let lock = Mutex::new(());
    let wt = cvar.wait_timeout(lock.lock().unwrap(), Duration::from_millis(1));
    let (_, wtr) = wt.unwrap();
    wtr
}

fn wtr_true() -> WaitTimeoutResult {
    let cvar = Condvar::new();
    let lock = Mutex::new(());
    let wt = cvar.wait_timeout(lock.lock().unwrap(), Duration::from_millis(0));
    let (_, wtr) = wt.unwrap();
    wtr
}

macro_rules! atomic {
    ($($type: ident, $base: ty);+) => {
        $(arbitrary!($type, SMapped<'a, $base, Self>;
            any_with_smap((), $type::new)
        );)+
    };
}

// impl_wrap_gen!(AtomicPtr); // We don't have impl Arbitrary for *mut T yet.
atomic!(AtomicBool, bool; AtomicIsize, isize; AtomicUsize, usize);

#[cfg(feature = "nightly")]
atomic!(AtomicI8, i8; AtomicI16, i16; AtomicI32, i32; AtomicI64, i64;
        AtomicU8, u8; AtomicU16, u16; AtomicU32, u32; AtomicU64, u64);

arbitrary!(Ordering,
    TupleUnion<(W<Just<Self>>, W<Just<Self>>, W<Just<Self>>,
                W<Just<Self>>, W<Just<Self>>)>;
    prop_oneof![
        Just(Ordering::Relaxed),
        Just(Ordering::Release),
        Just(Ordering::Acquire),
        Just(Ordering::AcqRel),
        Just(Ordering::SeqCst)
    ]
);

arbitrary!(RecvError; RecvError);

arbitrary!([T: Arbitrary<'a>] SendError<T>, SMapped<'a, T, Self>, T::Parameters;
    args => any_with_smap(args, SendError)
);

arbitrary!(RecvTimeoutError, TupleUnion<(W<Just<Self>>, W<Just<Self>>)>;
    prop_oneof![
        Just(RecvTimeoutError::Disconnected),
        Just(RecvTimeoutError::Timeout)
    ]
);

arbitrary!(TryRecvError, TupleUnion<(W<Just<Self>>, W<Just<Self>>)>;
    prop_oneof![
        Just(TryRecvError::Disconnected),
        Just(TryRecvError::Empty)
    ]
);

arbitrary!(
    [P: Clone + Default, T: Arbitrary<'a, Parameters = P>] TrySendError<T>,
    TupleUnion<(W<SMapped<'a, T, Self>>, W<SMapped<'a, T, Self>>)>, P;
    args => prop_oneof![
        any_with_smap(args.clone(), TrySendError::Disconnected),
        any_with_smap(args, TrySendError::Full),
    ]
);

#[cfg(feature = "nightly")]
generator!(Select, Select::new);

// If only half of a pair is generated then you will get a hang-up.
// Thus the only meaningful impls are in pairs.
arbitrary!([A] (Sender<A>, Receiver<A>), GenStrategy<Self>;
    GenStrategy::new(channel)
);

arbitrary!([A: Debug] (Sender<A>, IntoIter<A>), GenStrategy<Self>;
    GenStrategy::new(|| {
        let (rx, tx) = channel();
        (rx, tx.into_iter())
    })
);

arbitrary!([A] (SyncSender<A>, Receiver<A>), SMapped<'a, u32, Self>;
    static_map(any::<u32>(), |size| sync_channel(size as usize))
);

arbitrary!([A: Debug] (SyncSender<A>, IntoIter<A>), SMapped<'a, u32, Self>;
    static_map(any::<u32>(), |size| {
        let (rx, tx) = sync_channel(size as usize);
        (rx, tx.into_iter())
    })
);