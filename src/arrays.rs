
//==============================================================================
// Arrays:
//==============================================================================

use super::*;

// TODO: Consider allowing the user to pass a function which either mutates or
// accumulates the inner parameters for [0 -> N] in the strategy array.

macro_rules! impl_array {
    ($($n: expr),*) => {
        $(
            impl<'a, A: Arbitrary<'a>> Arbitrary<'a> for [A; $n] {
                valuetree!();
                type Parameters = A::Parameters;
                type Strategy = [A::Strategy; $n];
                fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
                    any_with::<[A; $n], _>(args)
                }
            }
        )*
    };
}

impl_array!(
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10,
    11,
    12,
    13,
    14,
    15,
    16,
    17,
    18,
    19,
    20,
    21,
    22,
    23,
    24,
    25,
    26,
    27,
    28,
    29,
    30,
    31
);
