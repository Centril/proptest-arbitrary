use std::string::*;

coarbitrary!(String; self, var =>
    var.nest(&self.capacity()).nest(&self.as_str()));

coarbitrary_unit!(FromUtf16Error);

#[cfg(feature = "unstable")]
coarbitrary!(FromUtf8Error; self, var =>
    var.nest(&self.as_bytes()).nest(&self.utf8_error()));

coarbitrary!(ParseError; self, _var => match *self {});