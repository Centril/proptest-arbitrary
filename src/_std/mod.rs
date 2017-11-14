//! Arbitrary implementations for `std`.

use super::*;

/*
//==============================================================================
// Alloc, i.e: Box, ...:
//==============================================================================

impl_smartptr!(Rc);
impl_smartptr!(Arc);
*/

mod ascii;

// TODO: Implement once it is possible to implement Arbitrary for &'a A`.
//mod borrow;

mod boxed;
mod cell;
mod char;
mod cmp;
mod collections;
mod convert;
mod env;
mod ffi;

/*
pub mod option;
pub mod result;
*/