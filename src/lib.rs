//-
// Copyright 2017 Mazdak Farrokhzad
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! NOTE: This version is still WIP; don't use yet, just reserving at crates.io.
//!
//! Proptest is a property testing framework (i.e., the [`QuickCheck`] family)
//! inspired by the [Hypothesis](http://hypothesis.works/) framework for
//! Python.
//!
//! This crate, `proptest-arbitrary`, additionally provides an [Arbitrary]
//! trait which allows you to have a canonical [Strategy] per type.
//! This is the equivalent of [Haskell QuickCheck's implementation of Arbitrary].
//! In this interpretation of Arbitray, Strategy is the equivalent of the Gen
//! monad.
//!
//! Arbitrary is currently implemented as:
//!
//! ```rust
//! # extern crate proptest;
//! # use std::fmt::Debug;
//! # use proptest::strategy::{Strategy, ValueTree};
//!
//! /// Arbitrary determines a canonical Strategy for the implementing type.
//! ///
//! /// ...
//! pub trait Arbitrary : Sized + Debug {
//!     /// Generates a Strategy for producing arbitrary values of type the
//!     /// implementing type (Self).
//!     fn arbitrary() -> Self::Strategy;
//!
//!     /// The type of ValueTree used for Self's Strategy.
//!     ///
//!     /// NOTE:
//!     /// This type should NOT be relied upon outside of this crate other than
//!     /// for implementing Arbitrary for other types.
//!     type ValueTree: ValueTree<Value = Self>;
//!
//!     /// The type of Strategy used to generate values of type Self.
//!     ///
//!     /// NOTE:
//!     /// This type should NOT be relied upon outside of this crate other than
//!     /// for implementing Arbitrary for other types.
//!     type Strategy: Strategy<Value = Self::ValueTree>;
//! }
//!
//! # fn main() {}
//! ```
//!
//! <!-- NOREADME
//! ## Status of this crate
//!
//! This crate is currently experimental. It will hopefully be included in
//! `proptest` in the future.
//!
//! The current definition of the [Arbitrary] trait might change in the future
//! pending the development of [existential types] in Rust.
//! However, as long as you don't rely on Arbitrary having associated types
//! in calling Arbitrary, in practice, this should not be a problem.
//!
//! This crate mostly just contains Arbitrary and implementations for it.
//! Hence, it is unlikely to see breaking change. If any change occurs, it will
//! likely be new implementations or newtypes around common types.
//!
//! See the [changelog] for a full list of substantial historical changes,
//! breaking and otherwise.
//!
//! NOREADME -->
//!
//! [changelog]:
//! https://github.com/Centril/proptest-arbitrary/blob/master/CHANGELOG.md
//!
//! [Arbitrary]: trait.Arbitrary.html
//!
//! [Strategy]:
//! https://docs.rs/proptest/0.3.0/proptest/strategy/trait.Strategy.html
//!
//! [existential types]: https://github.com/rust-lang/rfcs/pull/2071
//!
//! [Haskell QuickCheck's implementation of Arbitrary]:
//! https://hackage.haskell.org/package/QuickCheck/docs/Test-QuickCheck-Arbitrary.html
//!
//! [`QuickCheck`]:
//! https://hackage.haskell.org/package/QuickCheck

extern crate proptest;

use std::fmt::Debug;

use proptest::strategy::{Strategy, ValueTree};

//==============================================================================
// Arbitrary trait + auxilary functions:
//==============================================================================

/// Arbitrary determines a canonical [Strategy] for the implementing type.
///
/// It provides the function arbitrary which generates a Strategy for
/// producing arbitrary values of the implementing type *(Self)*.
///
/// This trait is the equivalent of
/// [Haskell QuickCheck's implementation of Arbitrary].
/// In this interpretation of Arbitray, Strategy is the equivalent of the Gen
/// monad.
///
/// [Strategy]: ../proptest/strategy/trait.Strategy.html
///
/// [Haskell QuickCheck's implementation of Arbitrary]:
/// https://hackage.haskell.org/package/QuickCheck/docs/Test-QuickCheck-Arbitrary.html
pub trait Arbitrary: Sized + Debug {
    /// Generates a [Strategy] for producing arbitrary values of type the
    /// implementing type (Self).
    ///
    /// [Strategy]: https://docs.rs/proptest/0.3.0/proptest/strategy/trait.Strategy.html
    fn arbitrary() -> Self::Strategy;

    /// The type of [ValueTree] used for Self's [Strategy].
    ///
    /// **NOTE:**
    /// This type should **NOT** be relied upon outside of this crate other than
    /// for implementing Arbitrary for other types.
    ///
    /// [ValueTree]: ../proptest/strategy/trait.ValueTree.html
    /// [Strategy]: ../proptest/strategy/trait.Strategy.html
    type ValueTree: ValueTree<Value = Self>;

    /// The type of [Strategy] used to generate values of type Self.
    ///
    /// **NOTE:**
    /// This type should **NOT** be relied upon outside of this crate other than
    /// for implementing Arbitrary for other types.
    ///
    /// [Strategy]: ../proptest/strategy/trait.Strategy.html
    type Strategy: Strategy<Value = Self::ValueTree>;
}

/// Generates a [Strategy] producing [Arbitrary] values of `T`.
/// Works better with type inference than [`any::<T>()`].
///
/// With this version, you shouldn't need to specify any of `T`, `S`, `V`.
/// This can have a positive effect on type inference.
/// However, if you want specify `T`, you should use [`any::<T>()`] instead.
///
/// # Example
///
/// The function can be used as:
///
/// ```rust
/// extern crate proptest_arbitrary;
/// use proptest_arbitrary::{arbitrary, Arbitrary};
///
/// fn gen_bool(x: bool) -> <bool as Arbitrary>::Strategy {
///     arbitrary()
/// }
///
/// # fn main() {}
/// ```
///
/// [`any::<T>()`]: fn.any.html
/// [Arbitrary]: trait.Arbitrary.html
/// [Strategy]: ../proptest/strategy/trait.Strategy.html
pub fn arbitrary<T, S, V>() -> S
where
    V: ValueTree<Value = T>,
    S: Strategy<Value = V>,
    T: Arbitrary<Strategy = S, ValueTree = V>,
{
    T::arbitrary()
}

/*
/// Generates a [Strategy] producing [Arbitrary] values of `T`.
/// Unlike [`arbitrary`], it should be used for being explicit on what `T` is.
///
/// Use this version instead of [`arbitrary`] if you want to be clear which
/// type you want to generate a Strategy for, or if you don't have an anchoring
/// type for type inference to work with.
///
/// # Example
///
/// The function can be used as:
///
/// ```rust
/// extern crate proptest_arbitrary;
/// use proptest_arbitrary::{arbitrary, Arbitrary};
///
/// fn gen_bool(x: bool) -> <bool as Arbitrary>::Strategy {
///     arbitrary()
/// }
///
/// # fn main() {}
/// ```
///
/// [`arbitrary`]: fn.arbitrary.html
/// [Arbitrary]: trait.Arbitrary.html
/// [Strategy]: ../proptest/strategy/trait.Strategy.html
pub fn any<T: Arbitrary>() -> <T as Arbitrary>::Strategy {
    T::arbitrary()
}

/// Generates a [Strategy] producing [Arbitrary] values of `T`.
/// This version boxes the Strategy, and thus you needn't specify `T`.
///
/// # Example
///
/// The function can be used as:
///
/// ```rust
/// extern crate proptest;
/// extern crate proptest_arbitrary;
///
/// use proptest::strategy::BoxStrategy;
/// use proptest_arbitrary::box_any;
///
/// fn gen_bool(x: bool) -> BoxedStrategy<bool> {
///     box_any()
/// }
/// ```
///
/// [`arbitrary`]: fn.arbitrary.html
/// [Arbitrary]: trait.Arbitrary.html
/// [Strategy]: ../proptest/strategy/trait.Strategy.html
pub fn box_any<T: Arbitrary>() -> BoxedStrategy<T>
where
    <T as Arbitrary>::Strategy: 'static
{
    any::<T>().boxed()
}
*/


/*

macro_rules! strat_of {
    ($type: ty) => {
        <bool as Arbitrary>::Strategy
    }
}

fn gen_bool(x: bool) -> <bool as Arbitrary>::Strategy {
    any::<bool>()
}

fn gen_bool2(x: bool) -> <bool as Arbitrary>::Strategy {
    arbitrary()
}


*/



//==============================================================================
// Basic macros:
//==============================================================================

// macro_rules! untype { $($x: tt)* => { $($x)* }; }

macro_rules! impl_arbitrary {
    ($self: ty, $st: ty, $logic: expr) => {
        impl Arbitrary for $self {
            type ValueTree = <$st as Strategy>::Value;
            type Strategy = $st;
            fn arbitrary() -> Self::Strategy { $logic }
        }
    };
}

macro_rules! impls {
    ($($self: ident),*) => {
        $(impl_arbitrary!($self, $self::Any, $self::ANY);)*
    };
}

//==============================================================================
// Primitive types:
//==============================================================================

use proptest::bool;
use proptest::num::{isize, usize, f32, f64, i16, i32, i64, i8, u16, u32, u64, u8};

impls! {
    bool, f32, f64,
    i8, i16, i32, i64, isize,
    u8, u16, u32, u64, usize
}

//==============================================================================
// Tuples:
//==============================================================================

/*

impl<A: Arbitrary, B: Arbitrary> Arbitrary for (A, B) {
    type ValueTree = <(A::Strategy, B::Strategy) as Strategy>::Value;
    type Strategy = (A::Strategy, B::Strategy);
    fn arbitrary() -> Self::Strategy {
        (arbitrary(), arbitrary())
    }
}

//impl_arbitrary!((), (), ());
*/
