//! CoArbitrary for ::std::collections.

use coarbitrary::*;

use std::cmp::*;

coarbitrary!([A: CoArbitrary] Reverse<A>; self, var => var.nest(&self.0));
coarbitrary!(Ordering; self, var => var.variant(match *self {
    Ordering::Greater => 0,
    Ordering::Equal => 1,
    Ordering::Less => 2,
}));