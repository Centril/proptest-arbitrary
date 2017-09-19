//==============================================================================
// Option + Result:
//==============================================================================

use super::*;

use proptest::option::{self, OptionStrategy};
use proptest::result::{self, MaybeOk};

impl_unary!(Option, OptionStrategy, => option::of(arbitrary()));
impl_binary!(Result, MaybeOk, => result::maybe_ok(arbitrary(), arbitrary()));

// TODO: Newtype for MaybeErr ?