use super::*;
use std::hash::{BuildHasherDefault, Hasher};
use std::collections::hash_map::{DefaultHasher, RandomState};

// NOTE: don't impl for std::hash::SipHasher.. since deprecated!

// over-constrain on purpose!
arbitrary!([H: Default + Hasher] BuildHasherDefault<H>; default());

generator!(DefaultHasher, default; RandomState, default);