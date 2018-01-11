//! Arbitrary implementations for `std::marker`.

use std::marker::PhantomData;
arbitrary!([T: ?Sized] PhantomData<T>; PhantomData);

#[cfg(test)]
mod test {
    no_panic_test!(phantom_data => PhantomData<u8>);
}