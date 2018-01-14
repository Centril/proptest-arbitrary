use coarbitrary::*;

use std::path::*;
use std::rc::Rc;
use std::sync::Arc;

impl<'a> CoArbitrary for &'a Path {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.as_os_str());
    }
}

impl<'a> CoArbitrary for &'a mut Path {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.as_os_str());
    }
}

impl CoArbitrary for Box<Path> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&&**self);
    }
}

impl CoArbitrary for Rc<Path> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&&**self);
    }
}

impl CoArbitrary for Arc<Path> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&&**self);
    }
}

impl CoArbitrary for PathBuf {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&&**self);
    }
}

impl<'a> CoArbitrary for StripPrefixError {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&());
    }
}

impl<'a> CoArbitrary for Display<'a> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&format!("{}", *self));
    }
}

delegate_iter!(['a] Iter<'a>);

impl<'a> CoArbitrary for PrefixComponent<'a> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.kind());
    }
}

impl<'a> CoArbitrary for Prefix<'a> {
    fn coarbitrary(&self, mut var: Perturbable) {
        match *self {
            Prefix::Verbatim(a) => var.variant(0).nest(&a),
            Prefix::VerbatimUNC(a, b) => var.variant(1).nest(&a).nest(&b),
            Prefix::VerbatimDisk(a) => var.variant(2).nest(&a),
            Prefix::DeviceNS(a) => var.variant(3).nest(&a),
            Prefix::UNC(a, b) => var.variant(4).nest(&a).nest(&b),
            Prefix::Disk(a) => var.variant(5).nest(&a),
        };
    }
}

impl<'a> CoArbitrary for Component<'a> {
    fn coarbitrary(&self, mut var: Perturbable) {
        match *self {
            Component::Prefix(a) => var.variant(0).nest(&a),
            Component::RootDir => var.variant(1),
            Component::CurDir => var.variant(2),
            Component::ParentDir => var.variant(3),
            Component::Normal(a) => var.variant(4).nest(&a),
        };
    }
}