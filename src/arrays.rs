//! Arbitrary implementations for arrays.

//==============================================================================
// Arrays:
//==============================================================================

use super::*;
use init_with::InitWith;
use std::mem;

/// A function taking `ParamsFor<A>` and transforming it. Allows
/// callers of `arbitrary_with` for arrays to mutate the parameters for each
/// element.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug,
         From)]
pub struct ParamModifier<A: Clone>(fn(usize, A) -> A);

impl<A: Clone> Default for ParamModifier<A> {
    fn default() -> Self {
        fn identity<A>(_: usize, x: A) -> A { x }
        ParamModifier(identity)
    }
}

impl<A: Clone> ParamModifier<A> {
    /// Creates a `ParamModifier` from a function `fn(usize, A) -> A`.
    pub fn new(fun: fn(usize, A) -> A) -> Self {
        ParamModifier(fun)
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

type ArrayParams<A> = product_type![ParamModifier<A>, A];

macro_rules! impl_array {
    ($($n: expr),*) => {
        $(
            impl<'a, A: Arbitrary<'a>> Arbitrary<'a> for [A; $n]
            where
                ParamsType<'a, A>: Clone
            {
                valuetree!();
                type Parameters = ArrayParams<A::Parameters>;
                type Strategy = [A::Strategy; $n];
                fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
                    let product_unpack![apm, mut curr] = args;
                    let mut i = 0;
                    <[A::Strategy; $n]>::init_with(|| {
                        let next = (apm.0)(i, curr.clone());
                        let new  = mem::replace(&mut curr, next);
                        i += 1;
                        any_with::<A>(new)
                    })
                }
            }
        )*
    };
}

impl_array!(
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10,
    11,
    12,
    13,
    14,
    15,
    16,
    17,
    18,
    19,
    20,
    21,
    22,
    23,
    24,
    25,
    26,
    27,
    28,
    29,
    30,
    31,
    32
);