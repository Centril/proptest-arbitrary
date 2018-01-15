use std::raw::*;

coarbitrary!(TraitObject; self, var =>
    var.nest(&(self.data as usize))
       .nest(&(self.vtable as usize)));