use super::*;
use std::ops::*;

arbitrary!(RangeFull; ..);
wrap_ctor!(RangeFrom, |a| a..);
wrap_ctor!(RangeTo, |a| ..a);

#[cfg(feature = "nightly")]
wrap_ctor!(RangeToInclusive, |a| ..=a);

#[cfg(feature = "nightly")]
arbitrary!(
    [A: PartialOrd + Arbitrary<'a>] RangeInclusive<A>,
    SMapped<'a, (A, A), Self>, product_type![A::Parameters, A::Parameters];
    args => any_with_smap(args, |(a, b)| if b < a { b..=a } else { a..=b })
);

arbitrary!(
    [A: PartialOrd + Arbitrary<'a>] Range<A>,
    SMapped<'a, (A, A), Self>, product_type![A::Parameters, A::Parameters];
    args => any_with_smap(args, |(a, b)| if b < a { b..a } else { a..b })
);

#[cfg(feature = "nightly")]
arbitrary!(
    [Y: Arbitrary<'a>, R: Arbitrary<'a>] GeneratorState<Y, R>,
    TupleUnion<(W<SMapped<'a, Y, Self>>, W<SMapped<'a, R, Self>>)>,
    product_type![Y::Parameters, R::Parameters];
    args => {
        let product_unpack![y, r] = args;
        prop_oneof![
            any_with_smap(y, GeneratorState::Yielded),
            any_with_smap(r, GeneratorState::Complete)
        ]
    }
);