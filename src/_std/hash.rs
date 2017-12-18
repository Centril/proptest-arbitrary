//! Arbitrary implementations for `std::hash`.

use super::*;
use std::hash::{BuildHasherDefault, Hasher};
use std::collections::hash_map::{DefaultHasher, RandomState};

// NOTE: don't impl for std::hash::SipHasher.. since deprecated!

// over-constrain on purpose!
arbitrary!([H: Default + Hasher] BuildHasherDefault<H>; default());

generator!(DefaultHasher, default; RandomState, default);

#[cfg(test)]
mod test {
    no_panic_test!(
        default_hasher => DefaultHasher,
        random_state => RandomState,
        build_hasher_default => BuildHasherDefault<DefaultHasher>
    );
}