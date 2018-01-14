//! CoArbitrary for ::std::option.

use coarbitrary::*;

use std::borrow::{ToOwned, Cow};

impl<'a, B> CoArbitrary for Cow<'a, B>
where
    B: CoArbitrary + 'a + ToOwned + ?Sized,
    B::Owned: CoArbitrary,
{
    fn coarbitrary(&self, mut var: Perturbable) {
        match *self {
            Cow::Borrowed(b) => var.variant(0).nest(b),
            Cow::Owned(ref o) => var.variant(1).nest(o),
        };
    }
}