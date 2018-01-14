//! Defines a tower of `Fn`-like traits.
//! This is used at the moment because the `Fn`-family traits
//! are unstable and can not be used in stable Rust.

/// Same semantics as `FnOnce`.
pub trait ApplyOnce<Args> {
    /// The output type of this closure.
    type Output;

    /// Calls the closure given args and returns a value of the output type.
    /// This consumes the closure.
    fn apply_once(self, args: Args) -> Self::Output;
}

/// Same semantics as `FnMut`.
pub trait ApplyMut<Args>: ApplyOnce<Args> {
    /// Calls the closure given args and returns a value of the output type.
    /// This does not consume the closure and allows
    /// the state of the closure to be modified.
    fn apply_mut(&mut self, args: Args) -> Self::Output;
}

/// Same semantics as `Fn`.
pub trait Apply<Args>: ApplyMut<Args> {
    /// Calls the closure given args and returns a value of the output type.
    /// This does not consume the closure and does not allow mutation of
    /// the closure's state.
    fn apply(&self, input: Args) -> Self::Output;

    /// Converts the closure into a boxed `Fn` closure.
    fn to_closure(self) -> Box<Fn(Args) -> Self::Output>
    where
        Args: 'static,
        Self::Output: 'static,
        Self: Sized + 'static,
    {
        Box::new(move |input| self.apply(input))
    }
}