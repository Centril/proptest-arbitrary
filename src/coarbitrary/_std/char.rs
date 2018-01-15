use std::char::*;

coarbitrary!(DecodeUtf16Error; self, var =>
    var.nest(&self.unpaired_surrogate()));

delegate_iter!([I: Iterator<Item = u16> + Clone] DecodeUtf16<I>);
delegate_iter!(EscapeDebug);
delegate_iter!(EscapeUnicode);

#[cfg(feature = "unstable")]
coarbitrary_unit!(::core::char::InvalidSequence, CharTryFromError);

#[cfg(feature = "unstable")]
delegate_iter!([I: Iterator<Item = u8> + Clone] DecodeUtf8<I>);

#[cfg(feature = "unstable")]
coarbitrary!(UnicodeVersion; self, var =>
    var.nest(&self.major).nest(&self.minor).nest(&self.micro));