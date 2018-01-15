use std::heap::*;

coarbitrary_unit!(CannotReallocInPlace, Heap, System);
coarbitrary!(Excess; self, var => var.nest(&(self.0 as usize)).nest(&self.1));
coarbitrary!(Layout; self, var => var.nest(&self.size()).nest(&self.align()));