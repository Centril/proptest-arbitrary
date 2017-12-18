//! Arbitrary implementations for `std::fmt`.

use std::fmt::Error;
arbitrary!(Error; Error);

#[cfg(test)]
mod test {
    no_panic_test!(error => Error);
}