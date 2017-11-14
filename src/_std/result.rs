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
pub type ResultParams<A, B> = Hlist![Probability, ResultBias, A, B];

impl<'a, A: Arbitrary<'a>, B: Arbitrary<'a>> Arbitrary<'a> for Result<A, B> {
    valuetree!();
    type Parameters = ResultParams<A::Parameters, B::Parameters>;
    type Strategy = ResultStrategy<A::Strategy, B::Strategy>;

    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        let hlist_pat![prob, bias, a, b] = args;
        let (p, aa, ab) = (prob.into(), arbitrary_with(a), arbitrary_with(b));
        match bias {
            ResultBias::Ok => SOk(result::maybe_ok_weighted(p, aa, ab)),
            ResultBias::Err => SErr(result::maybe_err_weighted(p, aa, ab)),
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