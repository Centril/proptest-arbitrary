# Proptest, Arbitrary

NOTE: This version is still WIP; don't use yet, just reserving at crates.io.

Proptest is a property testing framework (i.e., the [`QuickCheck`] family)
inspired by the [Hypothesis](http://hypothesis.works/) framework for
Python.

This crate, `proptest-arbitrary`, additionally provides an [Arbitrary]
trait which allows you to have a canonical [Strategy] per type.
This is the equivalent of [Haskell QuickCheck's implementation of Arbitrary].
In this interpretation of Arbitray, Strategy is the equivalent of the Gen
monad.

Arbitrary is currently implemented as:

```rust
# extern crate proptest;
# use std::fmt::Debug;
# use proptest::strategy::{Strategy, ValueTree};

/// Arbitrary determines a canonical Strategy for the implementing type.
///
/// ...
pub trait Arbitrary : Sized + Debug {
    /// Generates a Strategy for producing arbitrary values of type the
    /// implementing type (Self).
    fn arbitrary() -> Self::Strategy;

    /// The type of ValueTree used for Self's Strategy.
    ///
    /// NOTE:
    /// This type should NOT be relied upon outside of this crate other than
    /// for implementing Arbitrary for other types.
    type ValueTree: ValueTree<Value = Self>;

    /// The type of Strategy used to generate values of type Self.
    ///
    /// NOTE:
    /// This type should NOT be relied upon outside of this crate other than
    /// for implementing Arbitrary for other types.
    type Strategy: Strategy<Value = Self::ValueTree>;
}

# fn main() {}
```

## Status of this crate

This crate is currently experimental. It will hopefully be included in
`proptest` in the future.

The current definition of the [Arbitrary] trait might change in the future
pending the development of [existential types] in Rust.
However, as long as you don't rely on Arbitrary having associated types
in calling Arbitrary, in practice, this should not be a problem.

This crate mostly just contains Arbitrary and implementations for it.
Hence, it is unlikely to see breaking change. If any change occurs, it will
likely be new implementations or newtypes around common types.

See the [changelog] for a full list of substantial historical changes,
breaking and otherwise.


[changelog]:
https://github.com/Centril/proptest-arbitrary/blob/master/CHANGELOG.md

[Arbitrary]: trait.Arbitrary.html

[Strategy]:
https://docs.rs/proptest/0.3.0/proptest/strategy/trait.Strategy.html

[existential types]: https://github.com/rust-lang/rfcs/pull/2071

[Haskell QuickCheck's implementation of Arbitrary]:
https://hackage.haskell.org/package/QuickCheck/docs/Test-QuickCheck-Arbitrary.html

[`QuickCheck`]:
https://hackage.haskell.org/package/QuickCheck

# Acknowledgements

TODO

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
