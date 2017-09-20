//==============================================================================
// FromMapper:
//==============================================================================

use std::marker::PhantomData;
use std::fmt::Debug;

use proptest::strategy::{Strategy, ValueTree};
use proptest::strategy::statics::{Map, MapFn};

/// Do **NOT** use this type directly. This is a private implementation detail
/// that is unfortunately leaking, which might change in the future.
/// No guarantees are made regarding the stability of this type.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct FromMapper<I, O>(PhantomData<(I, O)>);

impl<I, O> Default for FromMapper<I, O> {
    fn default() -> Self {
        FromMapper(PhantomData)
    }
}

impl<I, O: From<I> + Debug> MapFn<I> for FromMapper<I, O> {
    type Output = O;

    fn apply(&self, val: I) -> Self::Output {
        val.into()
    }
}

pub(crate) type FromMapStrategy<S, I, O> = Map<S, FromMapper<I, O>>;

//==============================================================================
// FnMap + static_map:
//==============================================================================

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct FnMap<I, O, F: Fn(I) -> O>(F, PhantomData<(I, O)>);

impl<I, O, F> MapFn<I> for FnMap<I, O, F>
where
    O: Debug,
    F: Fn(I) -> O,
{
    type Output = O;

    fn apply(&self, x: I) -> Self::Output {
        self.0(x)
    }
}

pub(crate) type StaticMap<S, I, O, F> = Map<S, FnMap<I, O, F>>;

pub(crate) fn static_map<S, O, F>(
    strat: S,
    fun: F,
) -> StaticMap<S, <S::Value as ValueTree>::Value, O, F>
where
    S: Strategy,
    O: Debug,
    F: Fn(<S::Value as ValueTree>::Value) -> O,
{
    StaticMap::new(strat, FnMap(fun, PhantomData))
}

pub(crate) type W<T> = (u32, T);
