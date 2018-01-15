//! CoArbitrary for ::std::rc.

use coarbitrary::*;

use std::rc::*;

delegate_deref!([A: CoArbitrary + ?Sized] Rc<A>);
coarbitrary!([A: CoArbitrary + ?Sized] Weak<A>;
    self, var => var.nest(&self.upgrade()));