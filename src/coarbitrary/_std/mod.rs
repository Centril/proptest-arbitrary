//! Provides CoArbitrary impls for libstd.

mod any;
mod ascii;
mod boxed;
mod borrow;
mod cell;
mod char;
mod cmp;
mod collections;
mod convert;
mod env;
mod ffi;
mod fmt;
mod fs;
#[cfg(feature = "unstable")]
mod heap;
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
mod process;
mod rc;
mod result;
mod slice;
mod str;
mod string;
mod sync;
mod thread;
mod time;
#[cfg(feature = "unstable")]
mod raw;