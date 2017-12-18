//! Arbitrary implementations for `std::rc`.

use std::rc::Rc;

// Weak would always give None on upgrade since there's no owned Rc.

wrap_from!(Rc);

#[cfg(test)]
mod test {
    no_panic_test!(rc => Rc<u8>);
}