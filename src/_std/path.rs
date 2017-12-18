//! Arbitrary implementations for `std::path`.

use std::path::*;

// TODO: Figure out PathBuf and then Box/Rc/Box<Path>.

arbitrary!(StripPrefixError; Path::new("").strip_prefix("a").unwrap_err());

#[cfg(test)]
mod test {
    no_panic_test!(
        strip_prefix_error => StripPrefixError
    );
}