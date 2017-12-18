//! Arbitrary implementations for `std::ops`.

use super::*;
use std::ops::*;

arbitrary!(RangeFull; ..);
wrap_ctor!(RangeFrom, |a| a..);
wrap_ctor!(RangeTo, |a| ..a);

#[cfg(feature = "unstable")]
wrap_ctor!(RangeToInclusive, |a| ..=a);

#[cfg(feature = "unstable")]
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

#[cfg(feature = "unstable")]
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

#[cfg(test)]
mod test {
    no_panic_test!(
        range_full => RangeFull,
        range_from => RangeFrom<usize>,
        range_to   => RangeTo<usize>,
        range      => Range<usize>
    );

    #[cfg(feature = "unstable")]
    no_panic_test!(
        range_to_inclusive => RangeToInclusive<usize>,
        range_inclusive => RangeInclusive<usize>,
        generator_state => GeneratorState<u32, u64>
    );
}