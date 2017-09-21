//==============================================================================
// Option:
//==============================================================================

use super::*;
use proptest::option::{self, OptionStrategy};

/// A probability in the range [0.0, 1.0].
#[derive(Clone, PartialEq, Debug)]
pub struct Probability(f64);

impl From<f64> for Probability {
    fn from(x: f64) -> Self {
        Probability(x)
    }
}

impl Default for Probability {
    fn default() -> Self {
        0.5.into()
    }
}

params_unary!(OptionParams);

impl<'a, A: Arbitrary<'a>> Arbitrary<'a> for Option<A> {
    valuetree!();
    type Parameters = OptionParams<A::Parameters, Probability>;
    type Strategy = OptionStrategy<A::Strategy>;

    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        option::weighted(args.aux.0, arbitrary_with(args.a_params))
    }
}



/*


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

*/
