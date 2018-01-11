//! Arbitrary implementations for `std::fs`.

use super::*;
use std::fs::{DirBuilder};

// TODO: other parts (figure out workable semantics).

arbitrary!(DirBuilder, SMapped<bool, Self>; {
    static_map(any::<bool>(), |recursive| {
        let mut db = DirBuilder::new();
        db.recursive(recursive);
        db
    })
});

#[cfg(test)]
mod test {
    no_panic_test!(dir_builder => DirBuilder);
}