//! Arbitrary implementations for `std::boxed`.

wrap_from!(Box);

#[cfg(test)]
mod test {
    no_panic_test!(boxed => Box<u8>);
}