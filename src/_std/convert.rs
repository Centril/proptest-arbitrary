//! Arbitrary implementations for `std::convert`.

#[cfg(feature = "nightly")]
use std::convert::Infallible;

#[cfg(feature = "nightly")]
gen_strat!(Infallible, || panic!());