macro_rules! delegate_iter {
    ([$($bounds: tt)*] $typ: ty) => {
        delegate_iter!([$($bounds)*] $typ, clone);
    };
    ([$($bounds: tt)*] $typ: ty, $method: ident) => {
        impl<$($bounds)*> $crate::coarbitrary::CoArbitrary for $typ {
            fn coarbitrary(&self, var: $crate::coarbitrary::Perturbable) {
                $crate::coarbitrary::coarbitrary_iter(self.$method(), var);
            }
        }
    };
}

macro_rules! delegate_hash {
    ([$($bounds: tt)*] $typ: ty) => {
        impl<$($bounds)*> $crate::coarbitrary::CoArbitrary for $typ {
            fn coarbitrary(&self, var: $crate::coarbitrary::Perturbable) {
                $crate::coarbitrary::coarbitrary_hash(self, var);
            }
        }
    };
}

macro_rules! coarbitrary_unit {
    ($($typ: ty),*) => { $(
        impl $crate::coarbitrary::CoArbitrary for $typ {
            fn coarbitrary(&self, _: $crate::coarbitrary::Perturbable) {}
        }
    )* };
}

//==============================================================================
// Macros for testing:
//==============================================================================

#[cfg(test)]
macro_rules! coarbitrary_pure {
    ($($self: ident),*) => { coarbitrary_pure!($($self => $self),*); };
    ($($module: ident => $self: ty),*) => {
        $(
            mod $module {
                use coarbitrary::*;
                use rand::{weak_rng, Rng};

                proptest! {
                    #[test]
                    fn coarbitrary(ref x in $crate::any::<$self>()) {
                        let mut rng_1 = weak_rng();
                        let mut rng_2 = rng_1.clone();
                        x.coarbitrary(Perturbable::new(&mut rng_1));
                        x.coarbitrary(Perturbable::new(&mut rng_2));
                        // This is not really a good equality test,
                        // but we will get no false positives, and
                        // it is highly unlikely that it won't generate
                        // the same numbers.
                        prop_assert_eq!(rng_1.next_u64(), rng_2.next_u64());
                    }
                }
            }
        )*
    };
}