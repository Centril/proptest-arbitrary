//! CoArbitrary for ::std::result.

use coarbitrary::*;

use std::result::*;

coarbitrary!([A: CoArbitrary, B: CoArbitrary] Result<A, B>;
    self, var => match *self {
        Err(ref err) => var.variant(0).nest(err),
        Ok(ref ok) => var.variant(1).nest(ok),
    }
);
delegate_iter!([A: CoArbitrary + Clone] IntoIter<A>);
delegate_iter!(['a, A: CoArbitrary] Iter<'a, A>);