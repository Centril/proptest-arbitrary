use super::*;
use std::fs::{DirBuilder};

impl_arbitrary!(DirBuilder, SMapped<'a, bool, Self>, {
    static_map(any::<bool>(), |recursive| {
        let mut db = DirBuilder::new();
        db.recursive(recursive);
        db
    })
});

// TODO: other parts (figure out workable semantics).