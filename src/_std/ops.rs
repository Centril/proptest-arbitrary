use super::*;
use std::ops::*;

impl_just!(RangeFull, ..);
impl_wrap_gen!([] RangeFrom, |a| a..);
impl_wrap_gen!([] RangeTo, |a| ..a);

#[cfg(feature = "nightly")]
impl_wrap_gen!([] RangeToInclusive, |a| ..=a);

#[cfg(feature = "nightly")]
arbitrary_for!(
    [A: PartialOrd + Arbitrary<'a>] RangeInclusive<A>,
    SMapped<'a, (A, A), Self>, product_type![A::Parameters, A::Parameters],
    args => any_with_smap(args, |(a, b)| if b < a { b..=a } else { a..=b })
);

arbitrary_for!(
    [A: PartialOrd + Arbitrary<'a>] Range<A>,
    SMapped<'a, (A, A), Self>, product_type![A::Parameters, A::Parameters],
    args => any_with_smap(args, |(a, b)| if b < a { b..a } else { a..b })
);

#[cfg(feature = "nightly")]
arbitrary_for!(
    [Y: Arbitrary<'a>, R: Arbitrary<'a>] GeneratorState<Y, R>,
    TupleUnion<(W<SMapped<'a, Y, Self>>, W<SMapped<'a, R, Self>>)>,
    product_type![Y::Parameters, R::Parameters],
    args => {
        let product_unpack![y, r] = args;
        prop_oneof![
            any_with_smap(y, GeneratorState::Yielded),
            any_with_smap(r, GeneratorState::Complete)
        ]
    }
);