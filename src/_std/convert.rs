//! Arbitrary implementations for `std::convert`.
use std::convert::Infallible;

gen_strat!(Infallible, || panic!());