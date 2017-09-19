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

//==============================================================================
// Macros for quick implementing:
//==============================================================================

macro_rules! valuetree {
    () => {
        type ValueTree = <Self::Strategy as Strategy>::Value;
    };
}

macro_rules! impl_arbitrary {
    ($self: ty, $st: ty, $logic: expr) => {
        impl<'a> Arbitrary<'a> for $self {
            valuetree!();
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

macro_rules! impl_unary {
    ($typ: ident, $strat: ident, $($bound : path),* => $logic: expr) => {
        impl<'a, A: Arbitrary<'a> $(+ $bound)*> Arbitrary<'a> for $typ<A> {
            valuetree!();
            type Strategy = $strat<A::Strategy>;
            fn arbitrary() -> Self::Strategy {
                $logic
            }
        }
    };
}

macro_rules! impl_binary {
    ($typ: ident, $strat: ident, $($bound : path),* => $logic: expr) => {
        impl<'a, A: Arbitrary<'a> $(+ $bound)* , B: Arbitrary<'a>> Arbitrary<'a>
        for $typ<A, B> {
            valuetree!();
            type Strategy = $strat<A::Strategy, B::Strategy>;
            fn arbitrary() -> Self::Strategy {
                $logic
            }
        }
    };
}

//==============================================================================
// Primitive types:
//==============================================================================

use proptest::{bool, char};
use proptest::num::{isize, usize, f32, f64, i16, i32, i64, i8, u16, u32, u64, u8};

impls! {
    bool, f32, f64,
    i8, i16, i32, i64, isize,
    u8, u16, u32, u64, usize
}

impl_arbitrary!(char, char::CharStrategy<'a>, char::ANY);

//==============================================================================
// Option + Result:
//==============================================================================

use proptest::option::{self, OptionStrategy};
use proptest::result::{self, MaybeOk};

impl_unary!(Option, OptionStrategy, => option::of(arbitrary()));
impl_binary!(Result, MaybeOk, => result::maybe_ok(arbitrary(), arbitrary()));

//==============================================================================
// Collections:
//==============================================================================

use std::hash::Hash;
use std::vec::Vec;
use std::collections::{self, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

use proptest::collection::{self, binary_heap, btree_set, hash_map, hash_set, linked_list, vec,
                           vec_deque, BTreeSetStrategy, BinaryHeapStrategy, HashMapStrategy,
                           HashSetStrategy, LinkedListStrategy, VecDequeStrategy, VecStrategy};

impl_unary!(Vec, VecStrategy, => vec(arbitrary(), 0..100));
impl_unary!(VecDeque, VecDequeStrategy, => vec_deque(arbitrary(), 0..100));
impl_unary!(LinkedList, LinkedListStrategy, => linked_list(arbitrary(), 0..100));
impl_unary!(BTreeSet, BTreeSetStrategy, Ord => btree_set(arbitrary(), 0..100));
impl_unary!(BinaryHeap, BinaryHeapStrategy, Ord => {
    binary_heap(arbitrary(), 0..100)
});
impl_unary!(HashSet, HashSetStrategy, Hash, Eq => {
    hash_set(arbitrary(), 0..100)
});
impl_binary!(HashMap, HashMapStrategy, Hash, Eq => {
    hash_map(arbitrary(), arbitrary(), 0..100)
});

impl<'a, A, B> Arbitrary<'a> for collections::BTreeMap<A, B>
where
    A: Arbitrary<'static> + Ord,
    B: Arbitrary<'static>,
    <A as Arbitrary<'static>>::Strategy: 'static,
    <B as Arbitrary<'static>>::Strategy: 'static,
{
    valuetree!();
    type Strategy = collection::BTreeMapStrategy<A::Strategy, B::Strategy>;
    fn arbitrary() -> Self::Strategy {
        collection::btree_map(arbitrary(), arbitrary(), 0..100)
    }
}

//==============================================================================
// String:
//==============================================================================

use proptest::string::{string_regex_parsed, RegexGeneratorStrategy};

use regex_syntax::Expr::Concat;
use regex_syntax::Expr::*;
use regex_syntax::Repeater::ZeroOrMore;

impl_arbitrary!(String, RegexGeneratorStrategy<String>, {
    // Same as \\PC*
    string_regex_parsed(&Concat(vec![
        Literal {
            chars: vec!['\\', 'P'],
            casei: false,
        },
        Repeat {
            e: Box::new(Literal {
                chars: vec!['C'],
                casei: false,
            }),
            r: ZeroOrMore,
            greedy: true,
        },
    ])).unwrap()
});

//==============================================================================
// Tuples:
//==============================================================================

macro_rules! impl_tuple {
    ($($typ: ident),*) => {
        impl<'a, $($typ : Arbitrary<'a>),*> Arbitrary<'a> for ($($typ,)*) {
            valuetree!();
            type Strategy = ($($typ::Strategy,)*);
            fn arbitrary() -> Self::Strategy {
                ($(any::<$typ>()),*,)
            }
        }
    };
}

impl_tuple!(A);
impl_tuple!(A, B);
impl_tuple!(A, B, C);
impl_tuple!(A, B, C, D);
impl_tuple!(A, B, C, D, E);
impl_tuple!(A, B, C, D, E, F);
impl_tuple!(A, B, C, D, E, F, G);
impl_tuple!(A, B, C, D, E, F, G, H);
impl_tuple!(A, B, C, D, E, F, G, H, I);
impl_tuple!(A, B, C, D, E, F, G, H, I, J);

//==============================================================================
// Arrays:
//==============================================================================

macro_rules! impl_array {
    ($($n: expr),*) => {
        $(
            impl<'a, A: Arbitrary<'a>> Arbitrary<'a> for [A; $n] {
                valuetree!();
                type Strategy = [A::Strategy; $n];
                fn arbitrary() -> Self::Strategy {
                    any::<[A; $n]>()
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
    31
);

//==============================================================================
// Sandbox / Dummy region for trying stuff out first:
//==============================================================================
