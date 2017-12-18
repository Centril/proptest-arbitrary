//! Arbitrary implementations for `std::convert`.

// No sensible Arbitrary impl exists for void-like types like
// std::convert::Infallible.
//
// Auto-deriving should take care to simply not include such
// types in generation instead!