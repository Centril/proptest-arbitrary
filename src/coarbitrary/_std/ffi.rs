use std::ffi::*;
use std::rc::Rc;
use std::sync::Arc;

coarbitrary!(IntoStringError; self, var => var.nest(&self.utf8_error()));
coarbitrary!(NulError; self, var => var.nest(&self.nul_position()));
coarbitrary!(['a] &'a CStr; self, var => var.nest(&self.to_bytes_with_nul()));
coarbitrary!(['a] &'a mut CStr; self, var => var.nest(&self.to_bytes_with_nul()));
coarbitrary!(Box<CStr>; self, var => var.nest(&&**self));
coarbitrary!(Rc<CStr>; self, var => var.nest(&&**self));
coarbitrary!(Arc<CStr>; self, var => var.nest(&&**self));
coarbitrary!(CString; self, var => var.nest(&&**self));
coarbitrary!(['a] &'a OsStr; self, var => var.nest(&self.to_str()));
coarbitrary!(['a] &'a mut OsStr; self, var => var.nest(&self.to_str()));
coarbitrary!(Box<OsStr>; self, var => var.nest(&&**self));
coarbitrary!(Rc<OsStr>; self, var => var.nest(&&**self));
coarbitrary!(Arc<OsStr>; self, var => var.nest(&&**self));
coarbitrary!(OsString; self, var => var.nest(&&**self));