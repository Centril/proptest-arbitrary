//! Arbitrary implementations for primitive types.

//==============================================================================
// Primitive types:
//==============================================================================

use proptest::{bool, char};
use proptest::num::{isize, usize, f32, f64, i16, i32, i64, i8, u16, u32, u64, u8};

arbitrary!(
    bool, f32, f64,
    i8, i16, i32, i64, isize,
    u8, u16, u32, u64, usize
);

//==============================================================================
// Primitive types, char:
//==============================================================================

use std::borrow::Cow;

/// An inclusive char range from fst to snd.
/// TODO: replace with `std::ops::RangeInclusive<char>` once stabilized.
type CharRange = (char, char);
type CowSlices<T> = Cow<'static, [T]>;

const WHOLE_RANGE: &[CharRange] = &[('\x00', ::std::char::MAX)];

/// Equivalent to `proptest::char::ANY`.
impl Default for CharParam {
    fn default() -> Self {
        Self {
            special: Cow::Borrowed(char::DEFAULT_SPECIAL_CHARS),
            preferred: Cow::Borrowed(char::DEFAULT_PREFERRED_RANGES),
            ranges: Cow::Borrowed(WHOLE_RANGE),
        }
    }
}

/// Parameters to pass to `proptest::char::CharStrategy::new(..)`.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct CharParam {
    special: CowSlices<char>,
    preferred: CowSlices<CharRange>,
    ranges: CowSlices<CharRange>,
}

arbitrary!(char, char::CharStrategy<'static>, CharParam; args => {
    char::CharStrategy::new(args.special, args.preferred, args.ranges)
});

#[cfg(test)]
mod test {
    no_panic_test!(
        bool => bool,
        char => char,
        f32 => f32, f64 => f64,
        isize => isize, usize => usize,
        i8 => i8, i16 => i16, i32 => i32, i64 => i64,
        u8 => u8, u16 => u16, u32 => u32, u64 => u64
    );
}