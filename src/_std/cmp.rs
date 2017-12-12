//! Arbitrary implementations for `std::cmp`.

use super::*;
use std::cmp::{Reverse, Ordering};
use self::Ordering::*;
use proptest::strategy::{Just, TupleUnion};

impl_wrap_gen!([] Reverse, Reverse);

type WJO = W<Just<Ordering>>;
impl_arbitrary!(cmp::Ordering, TupleUnion<(WJO, WJO, WJO)>,
    prop_oneof![Just(Equal), Just(Less), Just(Greater)]
);