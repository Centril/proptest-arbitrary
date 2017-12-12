use std::hash::{BuildHasherDefault, Hasher};
use std::collections::hash_map::{DefaultHasher, RandomState};
use proptest::strategy::Just;

arbitrary_for!(
    [H: Default + Hasher] // over-constrain on purpose!
    BuildHasherDefault<H>, Just<Self>, (),
    _a => Just(BuildHasherDefault::default()));

// NOTE: don't impl for std::hash::SipHasher.. since deprecated!

gen_strat!(
    DefaultHasher, DefaultHasher::default;
    RandomState, RandomState::default);