//! Common types for parameters.

use super::*;

use std::ops::{Add, Range};
use proptest::num::f64;
use from_mapper::{SMapped, SFnPtrMap, static_map};

macro_rules! default {
    ($type: ty, $val: expr) => {
        impl Default for $type {
            fn default() -> Self { $val.into() }
        }
    };
}

//==============================================================================
// Probability, default = 0.5.
//==============================================================================

default!(Probability, 0.5);

impl From<f64> for Probability {
    fn from(prob: f64) -> Self {
        Probability::new(prob)
    }
}

impl Probability {
    /// Creates a `Probability` from a `f64`.
    /// 
    /// # Safety
    ///
    /// Panics if prob is outside interval `[0.0, 1.0]`
    pub fn new(prob: f64) -> Self {
        assert!(prob >= 0.0 && prob <= 1.0);
        Probability(prob)
    }
}

impl_arbitrary!(Probability, SFnPtrMap<Range<f64>, Self>, {
    static_map((0.0..1.0), Probability::new)
});

/// A probability in the range `[0.0, 1.0]` with default `0.5`.
#[derive(Clone, Copy, PartialEq, Debug, Generic, Into,
         Add, Sub, AddAssign, SubAssign, Mul, Div, Rem, Shr, Shl,
         MulAssign, DivAssign, RemAssign, ShrAssign, ShlAssign)]
pub struct Probability(f64);

//==============================================================================
// CollectionSizeBounds, default = 0..100.
//==============================================================================

default!(CollectionSizeBounds, (0..100));

type U2 = (usize, usize);

impl CollectionSizeBounds {
    /// Creates a `CollectionSizeBounds` from a `Range<usize>`.
    pub fn new(range: Range<usize>) -> Self {
        CollectionSizeBounds(range)
    }
}

/// Given `(low: usize, high: usize)`, then a range `(low..high)` is the result.
/// If the range is inverted, i.e: `low > high`,
/// then `low` and `high` are first swapped.
impl From<U2> for CollectionSizeBounds {
    fn from(x: U2) -> Self {
        let (mut low, mut high) = x;
        if low > high {
            ::std::mem::swap(&mut low, &mut high);
        }
        CollectionSizeBounds::new(low..high)
    }
}

/// Given `high: usize`, then a range `(0..high)` is the result.
impl From<usize> for CollectionSizeBounds {
    fn from(high: usize) -> Self {
        (0, high).into()
    }
}

/// Adds `usize` to both start and end of the bounds.
impl Add<usize> for CollectionSizeBounds {
    type Output = CollectionSizeBounds;

    fn add(self, rhs: usize) -> Self::Output {
        let Range { start, end } = self.into();
        Range {
            start: start + rhs,
            end: end + rhs
        }.into()
    }
}

impl_arbitrary!(CollectionSizeBounds, SMapped<'a, U2, Self>, {
    static_map(arbitrary(), <CollectionSizeBounds as From<U2>>::from)
});

/// The minimum and maximum bounds on the size of a collection.
/// The interval must form a subset of `[0, std::usize::MAX)`.
#[derive(Clone, PartialEq, Eq, Hash, Debug, Generic, From, Into)]
pub struct CollectionSizeBounds(Range<usize>);