//==============================================================================
// FromMapper:
//==============================================================================

use std::marker::PhantomData;
use std::fmt::Debug;

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