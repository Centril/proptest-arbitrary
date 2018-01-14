use coarbitrary::*;

use std::char::*;

impl CoArbitrary for DecodeUtf16Error {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.unpaired_surrogate());
    }
}

impl<I: Iterator<Item = u16> + Clone> CoArbitrary for DecodeUtf16<I> {
    fn coarbitrary(&self, var: Perturbable) {
        coarbitrary_iter((*self).clone(), var)
    }
}

impl CoArbitrary for EscapeDebug {
    fn coarbitrary(&self, var: Perturbable) {
        coarbitrary_iter(self.clone(), var)
    }
}

impl CoArbitrary for EscapeDefault {
    fn coarbitrary(&self, var: Perturbable) {
        coarbitrary_iter(self.clone(), var)
    }
}

impl CoArbitrary for EscapeUnicode {
    fn coarbitrary(&self, var: Perturbable) {
        coarbitrary_iter(self.clone(), var)
    }
}

#[cfg(feature = "unstable")]
coarbitrary_unit!(::core::char::InvalidSequence, CharTryFromError);

#[cfg(feature = "unstable")]
impl<I: Iterator<Item = u8> + Clone> CoArbitrary for DecodeUtf8<I> {
    fn coarbitrary(&self, var: Perturbable) {
        coarbitrary_iter((*self).clone(), var)
    }
}

#[cfg(feature = "unstable")]
impl CoArbitrary for UnicodeVersion {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.major).nest(&self.minor).nest(&self.micro);
    }
}