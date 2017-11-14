//! Arbitrary implementations for `std::ascii`.

use super::*;
use std::ascii;
use from_mapper::Mapped;

arbitrary_for!(ascii::EscapeDefault []
    [Mapped<'a, u8, ascii::EscapeDefault>]
    [ParamsType<'a, u8>],
    args => { any_with::<u8, _>(args).prop_map(ascii::escape_default) }
);