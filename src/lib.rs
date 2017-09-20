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
//! /// Arbitrary determines a canonical Strategy for the implementing type [..]
//! ///
//! /// [..]
//! pub trait Arbitrary<'a> : Sized + Debug {
//!    /// Generates a Strategy for producing arbitrary values of type the
//!    /// implementing type (Self) [..]
//!    fn arbitrary() -> Self::Strategy {
//!        Self::arbitrary_with(Default::default())
//!    }
//!
//!    /// Generates a Strategy for producing arbitrary values of type the
//!    /// implementing type (Self). The strategy is passed the arguments given
//!    /// in args [..].
//!    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy;
//!
//!    /// The type of parameters that arbitrary_with accepts for configuration
//!    /// of the generated Strategy. There must always be a Default way to
//!    /// construct arguments for the parameters [..]
//!    type Parameters: Default;
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

#[macro_use]
extern crate proptest;

extern crate bit_set;

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
    /// Calling this for the type `X` is the equivalent of using
    /// [`X::arbitrary_with(Default::default())`].
    ///
    /// [Strategy]: ../proptest/strategy/trait.Strategy.html
    ///
    /// [`X::arbitrary_with(Default::default())`]:
    ///     trait.Arbitrary.html#tymethod.arbitrary_with
    fn arbitrary() -> Self::Strategy {
        Self::arbitrary_with(Default::default())
    }

    /// Generates a [Strategy] for producing arbitrary values of type the
    /// implementing type (Self). The strategy is passed the arguments given
    /// in args.
    ///
    /// If you wish to use the [`default()`], use [`arbitrary`] instead.
    ///
    /// [Strategy]: ../proptest/strategy/trait.Strategy.html
    ///
    /// [`arbitrary`]: trait.Arbitrary.html#method.arbitrary
    ///
    /// [`default()`]:
    ///     https://doc.rust-lang.org/nightly/std/default/trait.Default.html
    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy;

    /// The type of parameters that [`arbitrary_with`] accepts for configuration
    /// of the generated [Strategy]. There must always be a [Default] way to
    /// construct arguments for the parameters.
    ///
    /// [`arbitrary_with`]: trait.Arbitrary.html#tymethod.arbitrary_with
    ///
    /// [Strategy]: ../proptest/strategy/trait.Strategy.html
    ///
    /// [`default()`]:
    ///     https://doc.rust-lang.org/nightly/std/default/trait.Default.html
    ///
    /// [Default]:
    ///     https://doc.rust-lang.org/nightly/std/default/trait.Default.html
    type Parameters: Default;

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

/// `StrategyFor` allows you to mention the type of [Strategy] for the input
/// type `A` without directly using associated types or without resorting to
/// existential types. This way, if implementation of [Arbitrary] changes, your
/// tests should not break. This can be especially beneficial when the type of
/// Strategy that you are dealing with is very long in name
/// (the case with generics). Additionally, if you have a custom Strategy type,
/// or use a Strategy type with generics in it where you've provided a custom
/// type for the type parameter, you need not export your type if `A` is
/// Arbitrary as the Strategy type is still reachable from `StrategyFor`.
///
/// This is the same as [StrategyType<'static, A>].
///
/// [Arbitrary]: trait.Arbitrary.html
/// [StrategyType<'static, A>]: type.StrategyType.html
/// [Strategy]: ../proptest/strategy/trait.Strategy.html
pub type StrategyFor<A> = StrategyType<'static, A>;

/// `StrategyType` allows you to mention the type of [Strategy] for the input
/// type `A` without directly using associated types or without resorting to
/// existential types. This way, if implementation of [Arbitrary] changes, your
/// tests should not break. This can be especially beneficial when the type of
/// Strategy that you are dealing with is very long in name
/// (the case with generics). Additionally, if you have a custom Strategy type,
/// or use a Strategy type with generics in it where you've provided a custom
/// type for the type parameter, you need not export your type if `A` is
/// Arbitrary as the Strategy type is still reachable from `StrategyType`.
///
/// Unless the strategy uses lifetimes in the type, you most likely want
/// [StrategyFor\<A\>] instead.
///
/// [Arbitrary]: trait.Arbitrary.html
/// [StrategyFor\<A\>]: type.StrategyFor.html
/// [Strategy]: ../proptest/strategy/trait.Strategy.html
pub type StrategyType<'a, A> = <A as Arbitrary<'a>>::Strategy;


/// `ParamsFor` allows you to mention the type of [Parameters] for the input
/// type `A` without directly using associated types or without resorting to
/// existential types. This way, if implementation of [Arbitrary] changes, your
/// tests should not break. Additionally, if you have a custom
/// `Arbitrary::Parameters` type, or use a `Arbitrary::Parameters` type with
/// generics in it where you've provided a custom type for the type parameter,
/// you need not export your type if `A` is Arbitrary as the Parameters type is
/// still reachable from `ParamsFor`.
///
/// This is the same as [ParamsType<'static, A>].
///
/// [Parameters]: trait.Arbitrary.html#associatedtype.Parameters
/// [Arbitrary]: trait.Arbitrary.html
/// [ParamsType<'static, A>]: type.StrategyType.html
/// [Strategy]: ../proptest/strategy/trait.Strategy.html
pub type ParamsFor<A> = ParamsType<'static, A>;

/// `ParamsType` allows you to mention the type of [Parameters] for the input
/// type `A` without directly using associated types or without resorting to
/// existential types. This way, if implementation of [Arbitrary] changes, your
/// tests should not break. Additionally, if you have a custom
/// `Arbitrary::Parameters` type, or use a `Arbitrary::Parameters` type with
/// generics in it where you've provided a custom type for the type parameter,
/// you need not export your type if `A` is Arbitrary as the Parameters type is
/// still reachable from `ParamsType`.
///
/// Unless the strategy uses lifetimes in the type, you most likely want
/// [ParamsFor\<A\>] instead.
///
/// [Parameters]: trait.Arbitrary.html#associatedtype.Parameters
/// [Arbitrary]: trait.Arbitrary.html
/// [ParamsFor\<A\>]: type.ParamsFor.html
/// [Strategy]: ../proptest/strategy/trait.Strategy.html
pub type ParamsType<'a, A> = <A as Arbitrary<'a>>::Parameters;

/// Generates a [Strategy] producing [Arbitrary] values of `A`.
/// Works better with type inference than [`any::<A>()`].
///
/// With this version, you shouldn't need to specify any of the (many) type
/// parameters explicitly. This can have a positive effect on type inference.
/// However, if you want specify `A`, you should use [`any::<A>()`] instead.
///
/// If you want to customize how the strategy is generated, use
/// [`arbitrary_with(args)`] where `args` are any arguments accepted by
/// `<A as Arbitrary>::Parameters` or any type `X` where
/// `<A as Arbitrary>::Parameters: From<X>`.
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
/// [Arbitrary]: trait.Arbitrary.html
/// [Strategy]: ../proptest/strategy/trait.Strategy.html
pub fn arbitrary<'a, A, S, V, P>() -> S
where
    P: Default,
    V: ValueTree<Value = A>,
    S: Strategy<Value = V>,
    A: Arbitrary<'a, Strategy = S, ValueTree = V, Parameters = P>,
{
    A::arbitrary()
}

/// Generates a [Strategy] producing [Arbitrary] values of `A` with the
/// given configuration arguments passed in `args`.
/// Works better with type inference than [`any_with::<A, _>(args)`].
///
/// With this version, you shouldn't need to specify any of the (many) type
/// parameters explicitly. This can have a positive effect on type inference.
/// However, if you want specify `A`, you should use
/// [`any_with::<A, _>(args)`] instead.
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
/// [`any_with::<A, _>(args)`]: fn.any_with.html
/// [`arbitrary()`]: fn.arbitrary.html
/// [Arbitrary]: trait.Arbitrary.html
/// [Strategy]: ../proptest/strategy/trait.Strategy.html
pub fn arbitrary_with<'a, PF, A, S, V, P>(args: PF) -> S
where
    P: Default + From<PF>,
    V: ValueTree<Value = A>,
    S: Strategy<Value = V>,
    A: Arbitrary<'a, Strategy = S, ValueTree = V, Parameters = P>,
{
    A::arbitrary_with(args.into())
}

/// Generates a [Strategy] producing [Arbitrary] values of `A`.
/// Unlike [`arbitrary`], it should be used for being explicit on what `A` is.
///
/// Use this version instead of [`arbitrary`] if you want to be clear which
/// type you want to generate a Strategy for, or if you don't have an anchoring
/// type for type inference to work with.
///
/// If you want to customize how the strategy is generated, use
/// [`any_with::<A>(args)`] where `args` are any arguments accepted by
/// the Arbitrary impl in question.
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
/// [Arbitrary]: trait.Arbitrary.html
/// [Strategy]: ../proptest/strategy/trait.Strategy.html
pub fn any<'a, A: Arbitrary<'a>>() -> StrategyType<'a, A> {
    A::arbitrary()
}

/// Generates a [Strategy] producing [Arbitrary] values of `A` with the
/// given configuration arguments passed in `args`. Unlike [`arbitrary_with`],
/// it should be used for being explicit on what `A` is.
///
/// Use this version instead of [`arbitrary_with`] if you want to be clear which
/// type you want to generate a Strategy for, or if you don't have an anchoring
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
///     any_with::<bool, _>(())
/// }
///
/// # fn main() {}
/// ```
/// [`any::<A>()`]: fn.any.html
/// [`arbitrary_with`]: fn.arbitrary_with.html
/// [Arbitrary]: trait.Arbitrary.html
/// [Strategy]: ../proptest/strategy/trait.Strategy.html
pub fn any_with<'a, A, PF>(args: PF) -> StrategyType<'a, A>
where
    A: Arbitrary<'a>,
    ParamsType<'a, A>: From<PF>,
{
    A::arbitrary_with(args.into())
}

/// Generates a [Strategy] producing [Arbitrary] values of `A`.
/// This version boxes the Strategy, and thus you needn't specify `A`.
///
/// If you want to customize how the strategy is generated, use
/// [`box_any_with(args)`] where `args` are any arguments accepted by
/// the Arbitrary impl in question.
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
/// [`box_any_with(args)`]: fn.box_any_with.html
/// [Arbitrary]: trait.Arbitrary.html
/// [Strategy]: ../proptest/strategy/trait.Strategy.html
pub fn box_any<A: Arbitrary<'static> + 'static>() -> BoxedStrategy<A>
where
    StrategyFor<A>: 'static,
{
    any::<A>().boxed()
}

/// Generates a [Strategy] producing [Arbitrary] values of `A` with the
/// given configuration arguments passed in `args`.
/// This version boxes the Strategy, and thus you needn't specify `A`.
///
/// If you don't want to specify any arguments and instead use the default
/// behavior, you should use [`box_any::<A>()`].
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
/// use proptest_arbitrary::box_any_with;
///
/// fn gen_bool(x: bool) -> BoxedStrategy<bool> {
///     box_any_with(())
/// }
///
/// # fn main() {}
/// ```
///
/// [`box_any::<A>()`]: fn.box_any.html
/// [Arbitrary]: trait.Arbitrary.html
/// [Strategy]: ../proptest/strategy/trait.Strategy.html
pub fn box_any_with<A, PF>(args: PF) -> BoxedStrategy<A>
where
    A: Arbitrary<'static> + 'static,
    ParamsFor<A>: From<PF>,
    StrategyFor<A>: 'static,
{
    any_with::<A, _>(args.into()).boxed()
}

mod utils;
use utils::*;
#[macro_use]
mod macros;
mod primitives;
pub mod option;
pub mod result;
mod from_mapper;
pub mod collections;
pub mod bits;
mod string;
mod arrays;
mod tuples;

//==============================================================================
// Sandbox / Dummy region for trying stuff out first:
//==============================================================================
