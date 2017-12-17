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
//! This crate, `proptest-arbitrary`, additionally provides an
//! [`Arbitrary`] trait which allows you to have a canonical [`Strategy`]
//! per type. This is the equivalent of [Haskell QuickCheck's implementation
//! of `Arbitrary`]. In this interpretation of `Arbitrary`, `Strategy` is the
//! equivalent of the `Gen` monad.
//!
//! Arbitrary is currently implemented as:
//!
//! ```ignore, rust
//! /// Arbitrary determines a canonical Strategy [..]
//! pub trait Arbitrary<'a> : Sized + Debug {
//!    fn arbitrary() -> Self::Strategy {
//!        Self::arbitrary_with(Default::default())
//!    }
//!
//!    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy;
//!
//!    type Parameters: Default;
//!
//!     type Strategy: Strategy<Value = Self::ValueTree>;
//!
//!     /// NOTE:
//!     /// This type should NOT be relied upon outside of this crate
//!     /// other than for implementing `Arbitrary` for other types.
//!     type ValueTree: ValueTree<Value = Self>;
//!
//! }
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
//! Therefore, it is unlikely to see breaking change. If any change occurs,
//! it will likely be new implementations or newtypes around common types.
//!
//! See the [changelog] for a full list of substantial historical changes,
//! breaking and otherwise.
//!
//! NOREADME -->
//!
//! [changelog]:
//! https://github.com/Centril/proptest-arbitrary/blob/master/CHANGELOG.md
//!
//! [`Arbitrary`]: trait.Arbitrary.html
//!
//! [`Strategy`]:
//! https://docs.rs/proptest/0.3.0/proptest/strategy/trait.Strategy.html
//!
//! [existential types]: https://github.com/rust-lang/rfcs/pull/2071
//!
//! [Haskell QuickCheck's implementation of `Arbitrary`]:
//! https://hackage.haskell.org/package/QuickCheck/docs/Test-QuickCheck-Arbitrary.html
//!
//! [`QuickCheck`]:
//! https://hackage.haskell.org/package/QuickCheck

#![deny(missing_docs)]

//==============================================================================
// Nightly opt-in features:
//==============================================================================

#![cfg_attr(feature = "nightly", feature(
      try_from
    , decode_utf8
    , io
    , iterator_step_by
    , ip
    , inclusive_range_syntax
    , inclusive_range
    , generator_trait
    , try_trait
    , integer_atomics
    , mpsc_select
    , thread_local_state
    , allocator_api
))]

//==============================================================================
// Frunk:
//==============================================================================

#[cfg(feature = "frunk")]
#[macro_use]
extern crate frunk_derives;

#[cfg(feature = "frunk")]
#[macro_use]
extern crate frunk_core;

#[cfg(feature = "frunk")]
#[macro_use] mod product_frunk;

#[cfg(not(feature = "frunk"))]
#[macro_use] mod product_tuple;

//==============================================================================
// Utility:
//==============================================================================

#[macro_use]
extern crate derive_more;

extern crate init_with;

//==============================================================================
// proptest:
//==============================================================================

#[macro_use]
extern crate proptest;

extern crate bit_set;

use std::fmt::Debug;
use proptest::strategy::*;

//==============================================================================
// Arbitrary trait + auxilary functions:
//==============================================================================

/// Arbitrary determines a canonical [`Strategy`] for the implementing type.
///
/// It provides the function arbitrary which generates a `Strategy` for
/// producing arbitrary values of the implementing type *(`Self`)*.
///
/// This trait is the equivalent of
/// [Haskell QuickCheck's implementation of `Arbitrary`][HaskellQC].
/// In this interpretation of `Arbitray`, `Strategy` is the equivalent of
/// the `Gen` monad.
///
/// `Arbitrary` currently only works for types which represent owned data as
/// opposed to borrowed data. This is a fundamental restriction of `proptest`
/// which may be lifted in the future as the [generic associated types (GAT)]
/// feature of Rust is implemented and stabilized.
///
/// [generic associated types (GAT)]: https://github.com/rust-lang/rust/issues/44265
///
/// [`Strategy`]: ../proptest/strategy/trait.Strategy.html
///
/// [HaskellQC]:
/// https://hackage.haskell.org/package/QuickCheck/docs/Test-QuickCheck-Arbitrary.html
pub trait Arbitrary<'a>: Sized + Debug {
    // Unfortunately, Generic Associated Types won't be in stable for some time.
    // Tracking issue: https://github.com/rust-lang/rust/issues/44265

    // We also can't get rid of `ValueTree` yet since it would require:
    // type Strategy: Strategy<Value = impl ValueTree<Value = Self>>;
    // which we can't express yet.

    /// Generates a [`Strategy`] for producing arbitrary values
    /// of type the implementing type (`Self`).
    ///
    /// Calling this for the type `X` is the equivalent of using
    /// [`X::arbitrary_with(Default::default())`].
    ///
    /// [`Strategy`]: ../proptest/strategy/trait.Strategy.html
    /// [`X::arbitrary_with(Default::default())`]:
    ///     trait.Arbitrary.html#tymethod.arbitrary_with
    fn arbitrary() -> Self::Strategy {
        Self::arbitrary_with(Default::default())
    }

    /// Generates a [`Strategy`] for producing arbitrary values of type the
    /// implementing type (`Self`). The strategy is passed the arguments given
    /// in args.
    ///
    /// If you wish to use the [`default()`] arguments,
    /// use [`arbitrary`] instead.
    ///
    /// [`Strategy`]: ../proptest/strategy/trait.Strategy.html
    ///
    /// [`arbitrary`]: trait.Arbitrary.html#method.arbitrary
    ///
    /// [`default()`]:
    ///     https://doc.rust-lang.org/nightly/std/default/trait.Default.html
    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy;

    /// The type of parameters that [`arbitrary_with`] accepts for configuration
    /// of the generated [`Strategy`]. Parameters must implement [`Default`].
    ///
    /// [`arbitrary_with`]: trait.Arbitrary.html#tymethod.arbitrary_with
    ///
    /// [`Strategy`]: ../proptest/strategy/trait.Strategy.html
    /// [`Default`]:
    ///     https://doc.rust-lang.org/nightly/std/default/trait.Default.html
    type Parameters: Default;

    /// The type of [`Strategy`] used to generate values of type `Self`.
    ///
    /// [`Strategy`]: ../proptest/strategy/trait.Strategy.html
    type Strategy: Strategy<Value = Self::ValueTree>;

    /// The type of [`ValueTree`] used for `Self`'s [`Strategy`].
    ///
    /// **NOTE:**
    /// This type should **NOT** be relied upon outside of this
    /// crate other than for implementing `Arbitrary` for other types.
    ///
    /// [`ValueTree`]: ../proptest/strategy/trait.ValueTree.html
    /// [`Strategy`]: ../proptest/strategy/trait.Strategy.html
    type ValueTree: ValueTree<Value = Self>;
}

/// `StrategyFor` allows you to mention the type of [`Strategy`] for the input
/// type `A` without directly using associated types or without resorting to
/// existential types. This way, if implementation of [`Arbitrary`] changes,
/// your tests should not break. This can be especially beneficial when the
/// type of `Strategy` that you are dealing with is very long in name
/// (the case with generics). Additionally, if you have a custom `Strategy`
/// type, or use a `Strategy` type with generics in it where you've provided a
/// custom type for the type parameter, you need not export your type if `A`
/// is `Arbitrary` as the `Strategy` type is still reachable from `StrategyFor`.
///
/// This is the same as [`StrategyType<'static, A>`].
///
/// [`Arbitrary`]: trait.Arbitrary.html
/// [`StrategyType<'static, A>`]: type.StrategyType.html
/// [`Strategy`]: ../proptest/strategy/trait.Strategy.html
pub type StrategyFor<A> = StrategyType<'static, A>;

/// `StrategyType` allows you to mention the type of [`Strategy`] for the
/// input type `A` without directly using associated types or without resorting
/// to existential types. This way, if implementation of [`Arbitrary`] changes,
/// your tests should not break. This can be especially beneficial when the
/// type of `Strategy` that you are dealing with is very long in name
/// (the case with generics). Additionally, if you have a custom `Strategy`
/// type, or use a `Strategy` type with generics in it where you've provided
/// a custom type for the type parameter, you need not export your type if `A`
/// is `Arbitrary` as the `Strategy` type is still reachable from `StrategyType`.
///
/// Unless the strategy uses lifetimes in the type, you most likely want
/// [`StrategyFor<A>`] instead.
///
/// [`Arbitrary`]: trait.Arbitrary.html
/// [`StrategyFor<A>`]: type.StrategyFor.html
/// [`Strategy`]: ../proptest/strategy/trait.Strategy.html
pub type StrategyType<'a, A> = <A as Arbitrary<'a>>::Strategy;

/// `ParamsFor` allows you to mention the type of [`Parameters`] for the input
/// type `A` without directly using associated types or without resorting to
/// existential types. This way, if implementation of [`Arbitrary`] changes,
/// your tests should not break. Additionally, if you have a custom
/// `Arbitrary::Parameters` type, or use a `Arbitrary::Parameters` type with
/// generics in it where you've provided a custom type for the type parameter,
/// you need not export your type if `A` is `Arbitrary` as the `Parameters`
/// type is still reachable from `ParamsFor`.
///
/// This is the same as [`ParamsType<'static, A>`].
///
/// [`Parameters`]: trait.Arbitrary.html#associatedtype.Parameters
/// [`Arbitrary`]: trait.Arbitrary.html
/// [`ParamsType<'static, A>`]: type.StrategyType.html
/// [`Strategy`]: ../proptest/strategy/trait.Strategy.html
pub type ParamsFor<A> = ParamsType<'static, A>;

/// `ParamsType` allows you to mention the type of [`Parameters`] for the input
/// type `A` without directly using associated types or without resorting to
/// existential types. This way, if implementation of [`Arbitrary`] changes,
/// your tests should not break. Additionally, if you have a custom
/// `Arbitrary::Parameters` type, or use a `Arbitrary::Parameters` type with
/// generics in it where you've provided a custom type for the type parameter,
/// you need not export your type if `A` is `Arbitrary` as the `Parameters`
/// type is still reachable from `ParamsType`.
///
/// Unless the strategy uses lifetimes in the type, you most likely want
/// [`ParamsFor<A>`] instead.
///
/// [`Parameters`]: trait.Arbitrary.html#associatedtype.Parameters
/// [`Arbitrary`]: trait.Arbitrary.html
/// [`ParamsFor<A>`]: type.ParamsFor.html
/// [`Strategy`]: ../proptest/strategy/trait.Strategy.html
pub type ParamsType<'a, A> = <A as Arbitrary<'a>>::Parameters;

/// Generates a [`Strategy`] producing [`Arbitrary`] values of `A`.
/// Works better with type inference than [`any::<A>()`].
///
/// With this version, you shouldn't need to specify any of the (many) type
/// parameters explicitly. This can have a positive effect on type inference.
/// However, if you want specify `A`, you should use [`any::<A>()`] instead.
///
/// For clarity, it is often a good idea to specify the type generated, and
/// so using [`any::<A>()`] can be a good idea.
///
/// If you want to customize how the strategy is generated, use
/// [`arbitrary_with(args)`] where `args` is of type
/// `<A as Arbitrary>::Parameters`.
///
/// # Example
///
/// The function can be used as:
///
/// ```rust
/// extern crate proptest_arbitrary;
/// use proptest_arbitrary::{arbitrary, StrategyFor};
///
/// fn gen_bool(x: bool) -> StrategyFor<bool> {
///     arbitrary()
/// }
///
/// # fn main() {}
/// ```
///
/// [`arbitrary_with(args)`]: fn.arbitrary_with.html
/// [`any::<A>()`]: fn.any.html
/// [`Arbitrary`]: trait.Arbitrary.html
/// [`Strategy`]: ../proptest/strategy/trait.Strategy.html
pub fn arbitrary<'a, A, S, V, P>() -> S
where
    P: Default,
    V: ValueTree<Value = A>,
    S: Strategy<Value = V>,
    A: Arbitrary<'a, Strategy = S, ValueTree = V, Parameters = P>,
{
    A::arbitrary()
}

/// Generates a [`Strategy`] producing [`Arbitrary`] values of `A` with the
/// given configuration arguments passed in `args`.
/// Works better with type inference than [`any_with::<A>(args)`].
///
/// With this version, you shouldn't need to specify any of the (many) type
/// parameters explicitly. This can have a positive effect on type inference.
/// However, if you want specify `A`, you should use
/// [`any_with::<A>(args)`] instead.
///
/// For clarity, it is often a good idea to specify the type generated, and
/// so using [`any::<A>()`] can be a good idea.
///
/// If you don't want to specify any arguments and instead use the default
/// behavior, you should use [`arbitrary()`].
///
/// # Example
///
/// The function can be used as:
///
/// ```rust
/// extern crate proptest_arbitrary;
/// use proptest_arbitrary::{arbitrary_with, StrategyFor};
///
/// fn gen_bool(x: bool) -> StrategyFor<bool> {
///     arbitrary_with(())
/// }
///
/// # fn main() {}
/// ```
///
/// [`any_with::<A>(args)`]: fn.any_with.html
/// [`arbitrary()`]: fn.arbitrary.html
/// [`Arbitrary`]: trait.Arbitrary.html
/// [`Strategy`]: ../proptest/strategy/trait.Strategy.html
pub fn arbitrary_with<'a, A, S, V, P>(args: P) -> S
where
    P: Default,
    V: ValueTree<Value = A>,
    S: Strategy<Value = V>,
    A: Arbitrary<'a, Strategy = S, ValueTree = V, Parameters = P>,
{
    A::arbitrary_with(args)
}

/// Generates a [`Strategy`] producing [`Arbitrary`] values of `A`.
/// Unlike [`arbitrary`], it should be used for being explicit on what `A` is.
/// For clarity, this may be a good idea.
///
/// Use this version instead of [`arbitrary`] if you want to be clear which
/// type you want to generate a `Strategy` for, or if you don't have an anchoring
/// type for type inference to work with.
///
/// If you want to customize how the strategy is generated, use
/// [`any_with::<A>(args)`] where `args` are any arguments accepted by
/// the `Arbitrary` impl in question.
///
/// # Example
///
/// The function can be used as:
///
/// ```rust
/// extern crate proptest_arbitrary;
/// use proptest_arbitrary::{any, StrategyFor};
///
/// fn gen_bool(x: bool) -> StrategyFor<bool> {
///     any::<bool>()
/// }
///
/// # fn main() {}
/// ```
///
/// [`any_with::<A>(args)`]: fn.any_with.html
/// [`arbitrary`]: fn.arbitrary.html
/// [`Arbitrary`]: trait.Arbitrary.html
/// [`Strategy`]: ../proptest/strategy/trait.Strategy.html
pub fn any<'a, A: Arbitrary<'a>>() -> StrategyType<'a, A> {
    A::arbitrary()
}

/// Generates a [`Strategy`] producing [`Arbitrary`] values of `A` with the
/// given configuration arguments passed in `args`. Unlike [`arbitrary_with`],
/// it should be used for being explicit on what `A` is.
/// For clarity, this may be a good idea.
///
/// Use this version instead of [`arbitrary_with`] if you want to be clear which
/// type you want to generate a `Strategy` for, or if you don't have an anchoring
/// type for type inference to work with.
///
/// If you don't want to specify any arguments and instead use the default
/// behavior, you should use [`any::<A>()`].
///
/// # Example
///
/// The function can be used as:
///
/// ```rust
/// extern crate proptest_arbitrary;
/// use proptest_arbitrary::{any_with, StrategyFor};
///
/// fn gen_bool(x: bool) -> StrategyFor<bool> {
///     any_with::<bool>(())
/// }
///
/// # fn main() {}
/// ```
/// [`any::<A>()`]: fn.any.html
/// [`arbitrary_with`]: fn.arbitrary_with.html
/// [`Arbitrary`]: trait.Arbitrary.html
/// [`Strategy`]: ../proptest/strategy/trait.Strategy.html
pub fn any_with<'a, A: Arbitrary<'a>>(args: A::Parameters)
    -> StrategyType<'a, A> {
    A::arbitrary_with(args)
}

//==============================================================================
// Modules:
//==============================================================================

#[macro_use] mod macros;

mod utils;
use utils::*;
pub use utils::{Mapped, FMapped as MappedF, SMapped as MappedS};
use extras::*;
mod extras;

mod params;
pub use params::*;

mod primitives;
pub use primitives::*;

pub mod bits;

mod _std;
pub use _std::*;

mod arrays;
pub use arrays::*;

mod tuples;

//==============================================================================
// Sandbox / Dummy region for trying stuff out first:
//==============================================================================.

// TODO: Relative likelyhood of union (optionally) based on a function:
// (lexical_pos: usize) -> likelyhood: usize.