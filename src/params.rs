//! Common types for parameters.

use super::*;

use std::ops::{Add, Range, RangeTo};
use proptest::num::f64;

//==============================================================================
// Probability, default = 0.5.
//==============================================================================

default!(Probability, 0.5);

/// Creates a `Probability` from some value that is convertible into it.
///
/// # Safety
///
/// Panics if the converted to probability would lie
/// outside interval `[0.0, 1.0]`. Consult the `Into` (or `From`)
/// implementation for more details.
pub fn prob<X: Into<Probability>>(from: X) -> Probability {
    from.into()
}

impl From<f64> for Probability {
    /// Creates a `Probability` from a `f64`.
    /// 
    /// # Safety
    ///
    /// Panics if the probability is outside interval `[0.0, 1.0]`.
    fn from(prob: f64) -> Self {
        Probability::new(prob)
    }
}

impl Probability {
    /// Creates a `Probability` from a `f64`.
    /// 
    /// # Safety
    ///
    /// Panics if the probability is outside interval `[0.0, 1.0]`.
    pub fn new(prob: f64) -> Self {
        assert!(prob >= 0.0 && prob <= 1.0);
        Probability(prob)
    }

    // Don't rely on these existing internally:

    /// Merges self together with some other argument producing a product
    /// type expected by some impelementations of `A: Arbitrary<'a>` in
    /// `A::Parameters`. This can be more ergonomic to work with and may
    /// help type inference.
    pub fn with<X>(self, and: X) -> product_type![Self, X] {
        product_pack![self, and]
    }

    /// Merges self together with some other argument generated with a
    /// default value producing a product type expected by some
    /// impelementations of `A: Arbitrary<'a>` in `A::Parameters`.
    /// This can be more ergonomic to work with and may help type inference.
    pub fn lift<X: Default>(self) -> product_type![Self, X] {
        self.with(default())
    }
}

arbitrary!(Probability, FromMapStrategy<Range<f64>, Self>;
    from_map_strategy(0.0..1.0)
);

#[cfg(feature = "frunk")]
use frunk_core::generic::Generic;

#[cfg(feature = "frunk")]
impl Generic for Probability {
    type Repr = f64;

    /// Converts the `Probability` into an `f64`.
    fn into(self) -> Self::Repr { self.0 }

    /// Creates a `Probability` from a `f64`.
    /// 
    /// # Safety
    ///
    /// Panics if the probability is outside interval `[0.0, 1.0]`.
    fn from(r: Self::Repr) -> Self { prob(r) }
}

/// A probability in the range `[0.0, 1.0]` with a default of `0.5`.
#[derive(Clone, Copy, PartialEq, Debug, Into)]
pub struct Probability(f64);

//==============================================================================
// SizeBounds, default = 0..100.
//==============================================================================

default!(SizeBounds, 0..100);

/// Creates a `SizeBounds` from some value that is convertible into it.
pub fn size_bounds<X: Into<SizeBounds>>(from: X) -> SizeBounds {
    from.into()
}

impl SizeBounds {
    /// Creates a `SizeBounds` from a `Range<usize>`.
    pub fn new(range: Range<usize>) -> Self {
        SizeBounds(range)
    }

    // Don't rely on these existing internally:

    /// Merges self together with some other argument producing a product
    /// type expected by some impelementations of `A: Arbitrary<'a>` in
    /// `A::Parameters`. This can be more ergonomic to work with and may
    /// help type inference.
    pub fn with<X>(self, and: X) -> product_type![Self, X] {
        product_pack![self, and]
    }

    /// Merges self together with some other argument generated with a
    /// default value producing a product type expected by some
    /// impelementations of `A: Arbitrary<'a>` in `A::Parameters`.
    /// This can be more ergonomic to work with and may help type inference.
    pub fn lift<X: Default>(self) -> product_type![Self, X] {
        self.with(default())
    }
}

/// Given `(low: usize, high: usize)`, then a range `[low..high)` is the result.
impl From<(usize, usize)> for SizeBounds {
    fn from(x: (usize, usize)) -> Self {
        (x.0..x.1).into()
    }
}

/// Given `exact`, then a range `[exact..exact + 1)` is the result.
impl From<usize> for SizeBounds {
    fn from(exact: usize) -> Self {
        size_bounds(exact..exact + 1)
    }
}

/// Given `..high`, then a range `[0..high)` is the result.
impl From<RangeTo<usize>> for SizeBounds {
    fn from(high: RangeTo<usize>) -> Self {
        size_bounds(0..high.end)
    }
}

/// Adds `usize` to both start and end of the bounds.
impl Add<usize> for SizeBounds {
    type Output = SizeBounds;

    fn add(self, rhs: usize) -> Self::Output {
        let Range { start, end } = self.0;
        size_bounds((start + rhs)..(end + rhs))
    }
}

arbitrary!(SizeBounds, FMapped<'a, Range<usize>, Self>;
    any_sinto::<Range<usize>, _>()
);

/// The minimum and maximum bounds on the size of a collection.
/// The interval must form a subset of `[0, std::usize::MAX)`.
#[derive(Clone, PartialEq, Eq, Hash, Debug, From, Into)]
#[cfg_attr(feature = "frunk", derive(Generic))]
pub struct SizeBounds(Range<usize>);