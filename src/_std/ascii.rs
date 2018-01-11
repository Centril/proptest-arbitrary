//! Arbitrary implementations for `std::ascii`.

use super::*;
use std::ascii::{EscapeDefault, escape_default};

arbitrary!(EscapeDefault, SMapped<u8, Self>, ParamsFor<u8>;
    args => any_with_smap(args, escape_default)
);

#[cfg(test)]
mod test {
    no_panic_test!(escape_default => EscapeDefault);
}