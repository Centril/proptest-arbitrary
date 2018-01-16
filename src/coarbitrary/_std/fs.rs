use std::fs::*;

coarbitrary!(DirEntry; self, var => var.nest(&self.path()));

// No sensible way to define CoArbitrary for File that is pure.
// Same for DirBuilder & ReadDir.

// We view a FileType as (bool, bool, bool).
coarbitrary!(FileType; self, var =>
    var.nest(&self.is_dir())
       .nest(&self.is_file())
       .nest(&self.is_symlink())
);

coarbitrary!(Permissions; self, var => var.nest(&self.readonly()));

#[cfg(feature = "unstable")]
coarbitrary!(Metadata; self, var => 
    var.nest(&self.file_type())
       .nest(&self.len())
       .nest(&self.permissions())
       .nest(&self.modified())
       .nest(&self.accessed())
       .nest(&self.created()));