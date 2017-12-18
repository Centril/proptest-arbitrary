//! Arbitrary implementations for `std::panic`.

use std::panic::AssertUnwindSafe;

wrap_ctor!(AssertUnwindSafe, AssertUnwindSafe);

#[cfg(test)]
mod test {
    no_panic_test!(assert_unwind_safe => AssertUnwindSafe<u8>);
}