//==============================================================================
// Option:
//==============================================================================

use super::*;
use proptest::option::{self, OptionStrategy};

/// Parameters for configuring the generation of `StrategyFor<Option<A>>`.
pub type OptionParams<A> = Hlist![Probability, A];

impl<'a, A: Arbitrary<'a>> Arbitrary<'a> for Option<A> {
    valuetree!();
    type Parameters = OptionParams<A::Parameters>;
    type Strategy = OptionStrategy<A::Strategy>;

    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        let hlist_pat![prob, a] = args;
        option::weighted(prob.into(), arbitrary_with(a))
    }
}