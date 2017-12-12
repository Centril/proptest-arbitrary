//! Common types for parameters.

use super::*;

use std::ops::{Add, Range, RangeTo};
use proptest::num::f64;

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
// SizeBounds, default = 0..100.
//==============================================================================

default!(SizeBounds, 0..100);

type U2 = (usize, usize);

impl SizeBounds {
    /// Creates a `SizeBounds` from a `Range<usize>`.
    pub fn new(range: Range<usize>) -> Self {
        SizeBounds(range)
    }

    pub (crate) fn and<X>(self, and: X) -> (Self, X) {
        (self, and)
    }
}

pub (crate) fn size_bounds<X>(from: X) -> SizeBounds
where
    SizeBounds: From<X> {
    SizeBounds::from(from)
}

/// Given `(low: usize, high: usize)`, then a range `[low..high)` is the result.
impl From<U2> for SizeBounds {
    fn from(x: U2) -> Self {
        (x.0..x.1).into()
    }
}

/// Given `exact`, then a range `[exact..exact + 1)` is the result.
impl From<usize> for SizeBounds {
    fn from(high: usize) -> Self {
        (high, high + 1).into()
    }
}

/// Given `..high`, then a range `[0..high)` is the result.
impl From<RangeTo<usize>> for SizeBounds {
    fn from(high: RangeTo<usize>) -> Self {
        (0, high.end).into()
    }
}

/// Adds `usize` to both start and end of the bounds.
impl Add<usize> for SizeBounds {
    type Output = SizeBounds;

    fn add(self, rhs: usize) -> Self::Output {
        let Range { start, end } = self.into();
        Range {
            start: start + rhs,
            end: end + rhs
        }.into()
    }
}

impl_arbitrary!(SizeBounds, SMapped<'a, U2, Self>, {
    static_map(any::<U2>(), <SizeBounds as From<U2>>::from)
});

/// The minimum and maximum bounds on the size of a collection.
/// The interval must form a subset of `[0, std::usize::MAX)`.
#[derive(Clone, PartialEq, Eq, Hash, Debug, Generic, From, Into)]
pub struct SizeBounds(Range<usize>);