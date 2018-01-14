use coarbitrary::*;

use std::ffi::*;
use std::rc::Rc;
use std::sync::Arc;

impl CoArbitrary for IntoStringError {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.utf8_error());
    }
}

impl CoArbitrary for NulError {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.nul_position());
    }
}

impl<'a> CoArbitrary for &'a CStr {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.to_bytes_with_nul());
    }
}

impl<'a> CoArbitrary for &'a mut CStr {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.to_bytes_with_nul());
    }
}

impl CoArbitrary for Box<CStr> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&&**self);
    }
}

impl CoArbitrary for Rc<CStr> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&&**self);
    }
}

impl CoArbitrary for Arc<CStr> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&&**self);
    }
}

impl CoArbitrary for CString {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&&**self);
    }
}

impl<'a> CoArbitrary for &'a OsStr {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.to_str());
    }
}

impl<'a> CoArbitrary for &'a mut OsStr {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.to_str());
    }
}

impl CoArbitrary for Box<OsStr> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&&**self);
    }
}

impl CoArbitrary for Rc<OsStr> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&&**self);
    }
}

impl CoArbitrary for Arc<OsStr> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&&**self);
    }
}

impl CoArbitrary for OsString {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&&**self);
    }
}