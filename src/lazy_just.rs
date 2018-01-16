//! Provides the `LazyJust` strategy.

use super::*;

use proptest::test_runner::TestRunner;

use std::fmt::{Debug, Formatter, Result as FResult};

/// A `Strategy` which always produces a single value value and never
/// simplifies. If `T` is `Clone`, you should use `Just` instead.
///
/// This is a generalization of `Just` and works by calling
/// the provided `Fn() -> T` in `.current()` every time. This is not a
/// very interesting strategy, but is required in cases where `T` is
/// not `Clone`. It is also used in `proptest_derive` where we can't
/// assume that your type is `Clone`.
pub struct LazyJust<T, F: Fn() -> T> {
    function: F
}

/// Shorthand for `LazyJust<T, fn() -> T>`.
pub type LazyJustFn<V> = LazyJust<V, fn() -> V>;

impl<T, F: Fn() -> T> LazyJust<T, F> {
    /// Constructs a `LazyJust` strategy.
    pub fn new(function: F) -> Self {
        Self { function }
    }
}

impl<T: Debug, F: Clone + Fn() -> T> Strategy for LazyJust<T, F> {
    type Value = Self;
    fn new_value(&self, _: &mut TestRunner) -> NewTree<Self> {
        Ok(self.clone())
    }
}

impl<V: Debug, F: Fn() -> V> ValueTree for LazyJust<V, F> {
    type Value = V;
    fn current(&self) -> Self::Value { (self.function)() }
    fn simplify(&mut self) -> bool { false }
    fn complicate(&mut self) -> bool { false }
}

impl<T, F: Copy + Fn() -> T> Copy for LazyJust<T, F> {}

impl<T, F: Clone + Fn() -> T> Clone for LazyJust<T, F> {
    fn clone(&self) -> Self {
        Self { function: self.function.clone() }
    }
}

impl<V, F: Fn() -> V> Debug for LazyJust<V, F> {
    fn fmt(&self, fmt: &mut Formatter) -> FResult {
        fmt.debug_struct("LazyJust")
           .field("function", &"<function>")
           .finish()
    }
}