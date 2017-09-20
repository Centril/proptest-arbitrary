//==============================================================================
// Result:
//==============================================================================

use super::*;
use proptest::result;

/// Bias for [`MaybeOk`] / [`MaybeErr`].
/// Default is `MaybeOk` which counts as "true".
///
/// [`MaybeOk`]: ../../proptest/result/struct.MaybeOk.html
/// [`MaybeErr`]: ../../proptest/result/struct.MaybeErr.html
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ResultBias {
    /// Use [`MaybeOk`](../../proptest/result/struct.MaybeOk.html).
    ///
    Ok,
    /// Use [`MaybeErr`](../../proptest/result/struct.MaybeErr.html).
    Err,
}

impl Default for ResultBias {
    fn default() -> Self {
        ResultBias::Ok
    }
}

impl From<bool> for ResultBias {
    fn from(is_ok: bool) -> Self {
        if is_ok {
            ResultBias::Ok
        } else {
            ResultBias::Err
        }
    }
}

/// Parameters for configuring the generation of `StrategyFor<Result<A, B>>`.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct ResultParams<A, B> {
    probability: f64,
    bias: ResultBias,
    a_params: A,
    b_params: B,
}

impl<A: Default, B: Default> Default for ResultParams<A, B> {
    fn default() -> Self {
        0.5.into()
    }
}

impl<A: Default, B: Default> From<()> for ResultParams<A, B> {
    fn from(_: ()) -> Self {
        Self::default()
    }
}

impl<A: Default, B: Default> From<f64> for ResultParams<A, B> {
    fn from(x: f64) -> Self {
        (x, def::<ResultBias>()).into()
    }
}

impl<AF, A: From<AF>, B: Default> From<(AF,)> for ResultParams<A, B> {
    fn from(x: (AF,)) -> Self {
        (0.5, def::<ResultBias>(), x.0).into()
    }
}

impl<RB, A, B> From<RB> for ResultParams<A, B>
where
    ResultBias: From<RB>,
    A: Default,
    B: Default,
{
    fn from(x: RB) -> Self {
        (0.5, x).into()
    }
}

impl<RB, A, B> From<(f64, RB)> for ResultParams<A, B>
where
    ResultBias: From<RB>,
    B: Default,
    A: Default,
{
    fn from(x: (f64, RB)) -> Self {
        (x.0, x.1, def()).into()
    }
}

impl<RB, AF, A, B> From<(f64, RB, AF)> for ResultParams<A, B>
where
    ResultBias: From<RB>,
    A: From<AF>,
    B: Default,
{
    fn from(x: (f64, RB, AF)) -> Self {
        (x.0, x.1, x.2, def()).into()
    }
}

impl<RB, AF, A, BF, B> From<(f64, RB, AF, BF)> for ResultParams<A, B>
where
    ResultBias: From<RB>,
    A: From<AF>,
    B: From<BF>,
{
    fn from(x: (f64, RB, AF, BF)) -> Self {
        ResultParams {
            probability: x.0,
            bias: x.1.into(),
            a_params: x.2.into(),
            b_params: x.3.into(),
        }
    }
}

use self::ResultValueTree::*;
use self::ResultStrategy::*;
use proptest::test_runner::TestRunner;

#[derive(Debug)]
pub enum ResultValueTree<A: ValueTree, B: ValueTree> {
    VOk(result::MaybeOkValueTree<A, B>),
    VErr(result::MaybeErrValueTree<A, B>),
}

#[derive(Debug)]
pub enum ResultStrategy<A: Strategy, B: Strategy> {
    SOk(result::MaybeOk<A, B>),
    SErr(result::MaybeErr<A, B>),
}

impl<A: ValueTree, B: ValueTree> ValueTree for ResultValueTree<A, B> {
    type Value = Result<A::Value, B::Value>;

    fn current(&self) -> Self::Value {
        match *self {
            VOk(ref v) => v.current(),
            VErr(ref v) => v.current(),
        }
    }

    fn simplify(&mut self) -> bool {
        match *self {
            VOk(ref mut v) => v.simplify(),
            VErr(ref mut v) => v.simplify(),
        }
    }

    fn complicate(&mut self) -> bool {
        match *self {
            VOk(ref mut v) => v.complicate(),
            VErr(ref mut v) => v.complicate(),
        }
    }
}

impl<A: Strategy, B: Strategy> Strategy for ResultStrategy<A, B> {
    type Value = ResultValueTree<A::Value, B::Value>;

    fn new_value(&self, runner: &mut TestRunner) -> Result<Self::Value, String> {
        Ok(match *self {
            SOk(ref s) => VOk(s.new_value(runner)?),
            SErr(ref s) => VErr(s.new_value(runner)?),
        })
    }
}

impl<'a, A: Arbitrary<'a>, B: Arbitrary<'a>> Arbitrary<'a> for Result<A, B> {
    valuetree!();
    type Parameters = ResultParams<A::Parameters, B::Parameters>;
    type Strategy = ResultStrategy<A::Strategy, B::Strategy>;

    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        match args.bias {
            ResultBias::Ok => SOk(result::maybe_ok_weighted(
                args.probability,
                arbitrary_with(args.a_params),
                arbitrary_with(args.b_params),
            )),
            ResultBias::Err => SErr(result::maybe_err_weighted(
                args.probability,
                arbitrary_with(args.a_params),
                arbitrary_with(args.b_params),
            )),
        }
    }
}
