//==============================================================================
// Option:
//==============================================================================

use super::*;
use proptest::option::{self, OptionStrategy};

/// Parameters for configuring the generation of `StrategyFor<Option<A>>`.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct OptionParams<A> {
    probability_of_some: f64,
    a_params: A,
}

impl<A: Default> Default for OptionParams<A> {
    fn default() -> Self {
        0.5.into()
    }
}

impl<A: Default> From<()> for OptionParams<A> {
    fn from(_: ()) -> Self {
        Self::default()
    }
}

impl<A: Default> From<f64> for OptionParams<A> {
    fn from(x: f64) -> Self {
        (x, def()).into()
    }
}

impl<AF, A: From<AF>> From<(AF,)> for OptionParams<A> {
    fn from(x: (AF,)) -> Self {
        (0.5, x.0).into()
    }
}

impl<AF, A: From<AF>> From<(f64, AF)> for OptionParams<A> {
    fn from(x: (f64, AF)) -> Self {
        Self {
            probability_of_some: x.0,
            a_params: x.1.into(),
        }
    }
}

impl<'a, A: Arbitrary<'a>> Arbitrary<'a> for Option<A> {
    valuetree!();
    type Parameters = OptionParams<A::Parameters>;
    type Strategy = OptionStrategy<A::Strategy>;

    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        option::weighted(args.probability_of_some, arbitrary_with(args.a_params))
    }
}
