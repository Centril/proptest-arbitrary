//! CoArbitrary for ::std::panic.

use coarbitrary::*;

use std::panic::*;

delegate_deref!([A: CoArbitrary] AssertUnwindSafe<A>);

#[cfg(feature = "unstable")]
coarbitrary!(['a] Location<'a>; self, var =>
    var.nest(&self.file())
       .nest(&self.line())
       .nest(&self.column()));