use super::*;
use std::fs::{DirBuilder};

// TODO: other parts (figure out workable semantics).

impl_arbitrary!(DirBuilder, SMapped<'a, bool, Self>, {
    static_map(any::<bool>(), |recursive| {
        let mut db = DirBuilder::new();
        db.recursive(recursive);
        db
    })
});