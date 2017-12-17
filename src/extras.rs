//! Extra strategies.

use super::*;
use std::fmt::{Debug, Formatter, Result as FResult};
use proptest::test_runner::TestRunner;

/// Shorthand for `Generator<V, fn() -> V>`.
pub type FnGenerator<V> = Generator<V, fn() -> V>;

/// Strategy for generating `V`s from an `Fn() -> V`.
/// It's not a very interesting Strategy, but required sometimes
/// when the only means of creating an object of some type is
/// via a function while type type is also not Clone.
#[derive(Clone, Copy)]
pub struct Generator<V, F: Fn() -> V> {
    generator: F,
}

impl<V, F: Fn() -> V> Generator<V, F> {
    /// Constructs a `Generator` strategy.
    pub fn new(generator: F) -> Self {
        Self { generator }
    }
}

impl<V: Debug, F: Clone + Fn() -> V> Strategy for Generator<V, F> {
    type Value = Self;
    fn new_value(&self, _: &mut TestRunner) -> Result<Self::Value, String> {
        Ok(Generator { generator: self.generator.clone() })
    }
}

impl<V: Debug, F: Fn() -> V> ValueTree for Generator<V, F> {
    type Value = V;
    fn current(&self) -> Self::Value { (self.generator)() }
    fn simplify(&mut self) -> bool { false }
    fn complicate(&mut self) -> bool { false }
}

impl<V, F: Fn() -> V> Debug for Generator<V, F> {
    fn fmt(&self, fmt: &mut Formatter) -> FResult {
        fmt.debug_struct("Generator")
           .field("generator", &"<function>")
           .finish()
    }
}