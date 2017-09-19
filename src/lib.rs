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
//! Python. This crate, `proptest-arbitrary`, additionally provides an
//! [Arbitrary] trait which allows you to have a canonical [Strategy] per type.
//! This is the equivalent of [Haskell QuickCheck's implementation of Arbitrary].
//! In this interpretation of Arbitrary, Strategy is the equivalent of the Gen
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
//! pub trait Arbitrary<'a> : Sized + Debug {
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
extern crate regex_syntax;

use std::fmt::Debug;

use proptest::strategy::{BoxedStrategy, Strategy, ValueTree};

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
pub trait Arbitrary<'a>: Sized + Debug {
    // Unfortunately, Generic Associated Types won't be in stable for some time.
    // Tracking issue: https://github.com/rust-lang/rust/issues/44265

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

/// If you want to be future proof, `StrategyOf` allows you to mention the
/// type of [Strategy] for the input type without directly using associated
/// types. This way, if implementation of [Arbitrary] changes, your tests
/// should not break.
///
/// This is the same as [StrategyType<'static, A>].
///
/// [Arbitrary]: trait.Arbitrary.html
/// [StrategyType<'static, A>]: type.StrategyType.html
/// [Strategy]: ../proptest/strategy/trait.Strategy.html
pub type StrategyOf<A> = StrategyType<'static, A>;

/// If you want to be future proof, `StrategyType` allows you to mention the
/// type of [Strategy] for the input type without directly using associated
/// types. This way, if implementation of [Arbitrary] changes, your tests
/// should not break.
///
/// Unless the strategy uses lifetimes in the type, you most likely want
/// [StrategyOf\<A\>] instead.
///
/// [Arbitrary]: trait.Arbitrary.html
/// [StrategyOf\<A\>]: type.StrategyOf.html
/// [Strategy]: ../proptest/strategy/trait.Strategy.html
pub type StrategyType<'a, A> = <A as Arbitrary<'a>>::Strategy;

/// Generates a [Strategy] producing [Arbitrary] values of `A`.
/// Works better with type inference than [`any::<A>()`].
///
/// With this version, you shouldn't need to specify any of `A`, `S`, `V`.
/// This can have a positive effect on type inference.
/// However, if you want specify `A`, you should use [`any::<A>()`] instead.
///
/// # Example
///
/// The function can be used as:
///
/// ```rust
/// extern crate proptest_arbitrary;
/// use proptest_arbitrary::{arbitrary, StrategyOf};
///
/// fn gen_bool(x: bool) -> StrategyOf<bool> {
///     arbitrary()
/// }
///
/// # fn main() {}
/// ```
///
/// [`any::<A>()`]: fn.any.html
/// [Arbitrary]: trait.Arbitrary.html
/// [Strategy]: ../proptest/strategy/trait.Strategy.html
pub fn arbitrary<'a, A, S, V>() -> S
where
    V: ValueTree<Value = A>,
    S: Strategy<Value = V>,
    A: Arbitrary<'a, Strategy = S, ValueTree = V>,
{
    A::arbitrary()
}

/// Generates a [Strategy] producing [Arbitrary] values of `A`.
/// Unlike [`arbitrary`], it should be used for being explicit on what `A` is.
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
/// use proptest_arbitrary::{any, StrategyOf};
///
/// fn gen_bool(x: bool) -> StrategyOf<bool> {
///     any::<bool>()
/// }
///
/// # fn main() {}
/// ```
///
/// [`arbitrary`]: fn.arbitrary.html
/// [Arbitrary]: trait.Arbitrary.html
/// [Strategy]: ../proptest/strategy/trait.Strategy.html
pub fn any<'a, A: Arbitrary<'a>>() -> StrategyType<'a, A> {
    A::arbitrary()
}

/// Generates a [Strategy] producing [Arbitrary] values of `A`.
/// This version boxes the Strategy, and thus you needn't specify `A`.
///
/// # Example
///
/// The function can be used as:
///
/// ```rust
/// extern crate proptest;
/// extern crate proptest_arbitrary;
///
/// use proptest::strategy::BoxedStrategy;
/// use proptest_arbitrary::box_any;
///
/// fn gen_bool(x: bool) -> BoxedStrategy<bool> {
///     box_any()
/// }
///
/// # fn main() {}
/// ```
///
/// [`arbitrary`]: fn.arbitrary.html
/// [Arbitrary]: trait.Arbitrary.html
/// [Strategy]: ../proptest/strategy/trait.Strategy.html
pub fn box_any<A: Arbitrary<'static> + 'static>() -> BoxedStrategy<A>
where
    StrategyOf<A>: 'static,
{
    any::<A>().boxed()
}

#[macro_use]
mod macros;
mod from_mapper;
mod primitives;
mod optional;
mod collections;
pub mod bits;
mod string;
mod tuples;
mod arrays;

//==============================================================================
// Sandbox / Dummy region for trying stuff out first:
//==============================================================================
