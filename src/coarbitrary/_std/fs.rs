use coarbitrary::*;

use std::fs::*;

impl CoArbitrary for DirEntry {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.path());
    }
}

// No sensible way to define CoArbitrary for File that is pure.
// Same for DirBuilder & ReadDir.

impl CoArbitrary for FileType {
    fn coarbitrary(&self, mut var: Perturbable) {
        // We view a FileType as (bool, bool, bool).
        var.nest(&self.is_dir())
           .nest(&self.is_file())
           .nest(&self.is_symlink());
    }
}

impl CoArbitrary for Permissions {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.readonly());
    }
}

impl CoArbitrary for Metadata {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.file_type())
           .nest(&self.len())
           .nest(&self.permissions())
           .nest(&self.modified())
           .nest(&self.accessed())
           .nest(&self.created());
    }
}