use std::marker::PhantomData;
use std::fmt::Debug;

use super::*;

use proptest::strategy::{Strategy, Map, ValueFor};
pub (crate) use proptest::strategy::statics::{
    Map as SMap,
    MapFn as SMapFn,
    //Filter as SFilter,
    //FilterFn as SFilterFn,
};

pub (super) fn any_with_map<'a, A, B, F>(args: A::Parameters, fun: F)
    -> Map<StrategyType<'a, A>, F>
where
    A: Arbitrary<'a>,
    B: Debug,
    F: Fn(A) -> B,
{
    any_with::<A>(args).prop_map(fun)
}

pub (super) fn any_with_sinto<'a, A, B>(args: A::Parameters)
    -> FMapped<'a, A, B>
where A: Arbitrary<'a>,
      B: Debug + From<A>
{
    from_map_strategy(any_with::<A>(args))
}

pub (super) fn any_sinto<'a, A, B>()
    -> FMapped<'a, A, B>
where A: Arbitrary<'a>,
      B: Debug + From<A>
{
    from_map_strategy(any::<A>())
}

pub (super) fn any_with_smap<'a, A, B>(args: A::Parameters, fun: fn(A) -> B)
    -> SMapped<'a, A, B>
where
    A: Arbitrary<'a>,
    B: Debug,
{
    static_map(any_with::<A>(args), fun)
}


/*

//==============================================================================
// Static Filter:
//==============================================================================

#[derive(Clone, Copy)]
pub struct StaticFilter<I>(fn(&I) -> bool);

impl<I> SFilterFn<I> for StaticFilter<I> {
    fn apply(&self, input: &I) -> bool {
        (self.0)(input)
    }
}

pub(crate) type FilterFnPtr<S> = SFilter<S, StaticFilter<ValueFor<S>>>;

pub(crate) fn static_filter<S: Strategy, W: AsRef<str>>(
    source: S, whence: W, filter: fn(&ValueFor<S>) -> bool
) -> FilterFnPtr<S> {
    SFilter::new(source, whence.as_ref().into(), StaticFilter(filter))
}
*/

//==============================================================================
// FromMapper:
//==============================================================================

/// Do **NOT** use this type directly. This is a private implementation detail
/// that is unfortunately leaking, which might change in the future.
/// No guarantees are made regarding the stability of this type.
#[derive(Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct FromMapper<I, O>(PhantomData<(I, O)>);

impl<I, O> Default for FromMapper<I, O> {
    fn default() -> Self { FromMapper(PhantomData) }
}

impl<I, O> Clone for FromMapper<I, O> {
    fn clone(&self) -> Self { Self::default() }
}

impl<I, O: From<I> + Debug> SMapFn<I> for FromMapper<I, O> {
    type Output = O;

    fn apply(&self, val: I) -> Self::Output {
        val.into()
    }
}

pub(crate) type FromMapStrategy<S, O> = SMap<S, FromMapper<ValueFor<S>, O>>;

pub (crate) fn from_map_strategy<S: Strategy, O>(strat: S)
    -> FromMapStrategy<S, O> {
    FromMapStrategy::new(strat, FromMapper::default())
}

/// A map from a strategy of `I` to `O` using the `From` trait for conversion.
pub type FMapped<'a, I, O> = FromMapStrategy<StrategyType<'a, I>, O>;

//==============================================================================
// FnMap + static_map:
//==============================================================================

#[derive(Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SFnMap<I, O>(fn(I) -> O);

impl<I, O> Clone for SFnMap<I, O> {
    fn clone(&self) -> Self { SFnMap(self.0) }
}

impl<I, O: Debug> SMapFn<I> for SFnMap<I, O> {
    type Output = O;
    fn apply(&self, x: I) -> Self::Output { self.0(x) }
}

pub(crate) type StaticMap<S, I, O> = SMap<S, SFnMap<I, O>>;

pub(crate) type SFnPtrMap<S, O> = SMap<S, SFnMap<ValueFor<S>, O>>;

pub(crate) fn static_map<S, O>(strat: S, fun: fn(ValueFor<S>) -> O)
    -> StaticMap<S, ValueFor<S>, O>
where
    S: Strategy,
    O: Debug
{
    StaticMap::new(strat, SFnMap(fun))
}

/// A static map from a strategy of `I` to `O`.
pub type SMapped<'a, I, O> = SMap<StrategyType<'a, I>, SFnMap<I, O>>;

//==============================================================================
// FnMap + static_map:
//==============================================================================

/// A normal map from a strategy of `I` to `O`.
pub type Mapped<'a, I, O> = Map<StrategyType<'a, I>, fn(I) -> O>;