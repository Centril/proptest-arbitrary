//==============================================================================
// String:
//==============================================================================

use super::*;

use proptest::string::{string_regex_parsed, RegexGeneratorStrategy};

use regex_syntax::Expr::*;
use regex_syntax::Repeater::ZeroOrMore;

impl_arbitrary!(String, RegexGeneratorStrategy<String>, {
    // Same as \\PC*
    string_regex_parsed(&Concat(vec![
        Literal {
            chars: vec!['\\', 'P'],
            casei: false,
        },
        Repeat {
            e: Box::new(Literal {
                chars: vec!['C'],
                casei: false,
            }),
            r: ZeroOrMore,
            greedy: true,
        },
    ])).unwrap()
});
