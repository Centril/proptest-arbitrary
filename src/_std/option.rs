//==============================================================================
// Option:
//==============================================================================

use super::*;
use std::option as opt;
use proptest::option::{self, OptionStrategy};

arbitrary_for!(
    [A: Arbitrary<'a>] Option<A>,
    OptionStrategy<A::Strategy>, Hlist![Probability, A::Parameters],
    args => {
        let hlist_pat![prob, a] = args;
        option::weighted(prob.into(), any_with::<A>(a))
    }
);

arbitrary_for!([A: Arbitrary<'a>] opt::IntoIter<A>,
    SMapped<'a, Option<A>, Self>, <Option<A> as Arbitrary<'a>>::Parameters,
    args => any_with_smap(args, Option::into_iter));

impl_just!(opt::NoneError, opt::NoneError);