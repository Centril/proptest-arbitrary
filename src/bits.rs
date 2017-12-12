//! Arbitrary implementations for integral types as bit patterns.

use super::*;

use std::mem;
use std::ops::Range;

use proptest::bits::{BitSetLike, BitSetStrategy};
use bit_set::BitSet;

use self::BitsParams::*;

/// Parameters for configuring the generation of `StrategyFor<Bits<A>>`.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum BitsParams<A> {
    /// Uses `BitSetStrategy::new(range.start, range.end)`.
    Ranged(Range<usize>),
    /// Uses `BitSetStrategy::masked(mask)`.
    Masked(A),
}

/// Yields the "all ones" bit pattern for self
/// for types which have such a notion.
pub trait AllOnes {
    /// Yields Self with all bits set.
    fn all_ones() -> Self;
}

macro_rules! allones_impls {
    ($($typ: ty),*) => {
        $(
            impl AllOnes for $typ {
                fn all_ones() -> Self { !(0 as $typ) }
            }
        )*
    };
}

allones_impls!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

impl<A: AllOnes + BitSetLike> Default for BitsParams<A> {
    fn default() -> Self {
        (A::all_ones(),).into()
    }
}

impl Default for BitsParams<BitSet> {
    /// Uses a range: `0 .. sizeof(usize)^2`.
    fn default() -> Self {
        Ranged(0..(mem::size_of::<usize>() * 8).pow(2))
    }
}

impl<A: BitSetLike> From<(A,)> for BitsParams<A> {
    fn from(x: (A,)) -> Self {
        Masked(x.0)
    }
}

impl<A> From<Range<usize>> for BitsParams<A> {
    fn from(x: Range<usize>) -> Self {
        Ranged(x)
    }
}

impl<T: BitSetLike> Bits<T> {
    /// Wraps an integral type into `Bits` which views it as a bit pattern.
    pub fn new(t: T) -> Self {
        t.into()
    }
}

/// Bits is a simple newtype for treating the generic type parameter `T` as
/// a set of bits for the purposes of production of arbitrary values.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug,
         Generic, From,
         Not, Neg, BitAnd, BitOr, BitXor,
         Add, Sub, AddAssign, SubAssign, Mul, Div, Rem, Shr, Shl)]
pub struct Bits<T: BitSetLike>(T);

macro_rules! impl_bits {
    ($($typ: ty),*) => {
        $(
            impl From<Bits<Self>> for $typ {
                fn from(x: Bits<Self>) -> Self { x.0 }
            }

            arbitrary_for!([] Bits<$typ>,
                FromMapStrategy<BitSetStrategy<$typ>, Self>,
                BitsParams<$typ>,
                args => from_map_strategy(match args {
                    Ranged(r) => BitSetStrategy::new(r.start, r.end),
                    Masked(m) => BitSetStrategy::masked(m),
                })
            );
        )*
    };
}

impl_bits!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, BitSet);
