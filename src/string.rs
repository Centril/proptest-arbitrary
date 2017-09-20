//==============================================================================
// String:
//==============================================================================

use super::*;

use proptest::string::{string_regex, RegexGeneratorStrategy};

impl<'a> Arbitrary<'a> for String {
    valuetree!();

    type Parameters = &'a str;
    type Strategy = RegexGeneratorStrategy<String>;

    /// ## Safety
    ///
    /// This implementation panics if the input is not a valid regex proptest
    /// can handle.
    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        string_regex(args).unwrap()
    }
}
