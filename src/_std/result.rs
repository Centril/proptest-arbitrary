//==============================================================================
// Result:
//==============================================================================

use super::*;
use std::result::IntoIter;
use proptest::result;
use frunk_core::hlist::LiftInto;

// We assume that `MaybeOk` is canonical as it's the most likely Strategy
// a user wants.

arbitrary_for!([A: Arbitrary<'a>, B: Arbitrary<'a>] Result<A, B>,
    result::MaybeOk<A::Strategy, B::Strategy>,
    Hlist![Probability, A::Parameters, B::Parameters],
    args => {
        let hlist_pat![prob, a, b] = args;
        let (p, a, b) = (prob.into(), any_with::<A>(a), any_with::<B>(b));
        result::maybe_ok_weighted(p, a, b)
    }
);

arbitrary_for!([A: Arbitrary<'a>] IntoIter<A>,
    SMapped<'a, Result<A, ()>, Self>, A::Parameters,
    args => any_with_smap(args.lift_into(), Result::into_iter));