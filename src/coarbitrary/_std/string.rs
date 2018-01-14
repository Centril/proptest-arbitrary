use coarbitrary::*;

use std::string::*;

impl CoArbitrary for String {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.capacity()).nest(&self.as_str());
    }
}

impl CoArbitrary for FromUtf16Error {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&());
    }
}

#[cfg(feature = "unstable")]
impl CoArbitrary for FromUtf8Error {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.as_bytes()).nest(&self.utf8_error());
    }
}

impl CoArbitrary for ParseError {
    fn coarbitrary(&self, _: Perturbable) { match *self {} }
}