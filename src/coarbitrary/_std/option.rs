//! CoArbitrary for ::std::option.

use coarbitrary::*;

use std::option::*;

coarbitrary!([A: CoArbitrary] Option<A>; self, var => match *self {
    None => var.variant(0),
    Some(ref x) => var.variant(1).nest(x),
});

delegate_iter!([A: Clone + CoArbitrary] IntoIter<A>);
delegate_iter!(['a, A: CoArbitrary] Iter<'a, A>);

#[cfg(feature = "unstable")]
coarbitrary_unit!(NoneError);