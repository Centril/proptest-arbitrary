//! Arbitrary implementations for `std::cmp`.

use super::*;
use std::cmp::{Reverse, Ordering};
use self::Ordering::*;

wrap_ctor!(Reverse, Reverse);

type WJO = W<Just<Ordering>>;
arbitrary!(cmp::Ordering, TupleUnion<(WJO, WJO, WJO)>;
    prop_oneof![Just(Equal), Just(Less), Just(Greater)]
);