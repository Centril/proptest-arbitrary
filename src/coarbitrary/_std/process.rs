use std::process::*;

// This OK -  .id()  is a pure function even tho everything else in
// Child is impure.
coarbitrary!(Child; self, var => var.nest(&self.id()));

coarbitrary!(ExitStatus; self, var => var.nest(&self.code()));

coarbitrary!(Output; self, var =>
    var.nest(&self.status)
        .nest(&self.stdout)
        .nest(&self.stderr));