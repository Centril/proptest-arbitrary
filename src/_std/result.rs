//==============================================================================
// Result:
//==============================================================================

use super::*;
use std::result::IntoIter;
use proptest::result;

// We assume that `MaybeOk` is canonical as it's the most likely Strategy
// a user wants.

arbitrary!([A: Arbitrary<'a>, B: Arbitrary<'a>] Result<A, B>,
    result::MaybeOk<A::Strategy, B::Strategy>,
    product_type![Probability, A::Parameters, B::Parameters];
    args => {
        let product_unpack![prob, a, b] = args;
        let (p, a, b) = (prob.into(), any_with::<A>(a), any_with::<B>(b));
        result::maybe_ok_weighted(p, a, b)
    }
);

arbitrary!([A: Arbitrary<'a>] IntoIter<A>,
    SMapped<'a, Result<A, ()>, Self>,
    <Result<A, ()> as Arbitrary<'a>>::Parameters;
    args => any_with_smap(args, Result::into_iter)
);