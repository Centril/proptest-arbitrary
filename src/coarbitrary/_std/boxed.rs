//! CoArbitrary for ::std::boxed.

use coarbitrary::*;

delegate_deref!([A: CoArbitrary + ?Sized] Box<A>);