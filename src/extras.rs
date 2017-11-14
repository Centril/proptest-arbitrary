//! Extra strategies.

use super::*;

/// Strategy for generating `V`s from a function.
/// It's not a very interesting Strategy, but required sometimes.
#[derive(Debug, Clone, Copy)]
pub struct GenStrategy<V>(fn() -> V);

impl<V> GenStrategy<V> {
    /// Constructs a `GenStrategy`.
    pub fn new(fun: fn() -> V) -> Self {
        GenStrategy(fun)
    }
}

use proptest::test_runner::TestRunner;

impl<V: Debug> Strategy for GenStrategy<V> {
    type Value = Self;
    fn new_value(&self, _: &mut TestRunner) -> Result<Self::Value, String> {
        Ok(GenStrategy(self.0))
    }
}

impl<V: Debug> ValueTree for GenStrategy<V> {
    type Value = V;
    fn current(&self) -> Self::Value { (self.0)() }
    fn simplify(&mut self) -> bool { false }
    fn complicate(&mut self) -> bool { false }
}