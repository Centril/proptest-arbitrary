//! Arbitrary implementations for `std::ascii`.

use super::*;
use std::ascii::{EscapeDefault, escape_default};

arbitrary_for!([] EscapeDefault, SMapped<'a, u8, Self>, ParamsType<'a, u8>,
    args => any_with_smap(args, escape_default)
);