//==============================================================================
// Option:
//==============================================================================

use super::*;
use std::option as opt;
use proptest::option::{self, OptionStrategy};

arbitrary!(
    [A: Arbitrary<'a>] opt::Option<A>,
    OptionStrategy<A::Strategy>,
    product_type![Probability, A::Parameters];
    args => {
        let product_unpack![prob, a] = args;
        option::weighted(prob.into(), any_with::<A>(a))
    }
);

arbitrary!([A: Arbitrary<'a>] opt::IntoIter<A>,
    SMapped<'a, opt::Option<A>, Self>,
    <opt::Option<A> as Arbitrary<'a>>::Parameters;
    args => any_with_smap(args, Option::into_iter));

#[cfg(feature = "nightly")]
arbitrary!(opt::NoneError; opt::NoneError);