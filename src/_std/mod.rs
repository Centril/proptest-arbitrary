//! Arbitrary implementations for libstd.

use super::*;

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
mod fmt;
mod fs;
mod hash;
mod io;
mod iter;
mod marker;
mod mem;
mod net;
mod num;
mod ops;
mod option;
mod panic;
mod path;
mod rc;
mod result;
mod str;
mod string;
pub use self::string::*;
mod sync;
mod thread;
mod time;

#[cfg(feature = "nightly")]
mod heap;