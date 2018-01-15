use std::path::*;
use std::rc::Rc;
use std::sync::Arc;

coarbitrary!(['a] &'a Path; self, var => var.nest(&self.as_os_str()));
coarbitrary!(['a] &'a mut Path; self, var => var.nest(&self.as_os_str()));
coarbitrary!(Box<Path>; self, var => var.nest(&&**self));
coarbitrary!(Rc<Path>; self, var => var.nest(&&**self));
coarbitrary!(Arc<Path>; self, var => var.nest(&&**self));
coarbitrary!(PathBuf; self, var => var.nest(&&**self));
coarbitrary!(['a] StripPrefixError; self, _var => {});
coarbitrary!(['a] Display<'a>; self, var => var.nest(&format!("{}", *self)));
delegate_iter!(['a] Iter<'a>);
coarbitrary!(['a] PrefixComponent<'a>; self, var => var.nest(&self.kind()));
coarbitrary!(['a] Prefix<'a>; self, var => match *self {
    Prefix::Verbatim(a) => var.variant(0).nest(&a),
    Prefix::VerbatimUNC(a, b) => var.variant(1).nest(&a).nest(&b),
    Prefix::VerbatimDisk(a) => var.variant(2).nest(&a),
    Prefix::DeviceNS(a) => var.variant(3).nest(&a),
    Prefix::UNC(a, b) => var.variant(4).nest(&a).nest(&b),
    Prefix::Disk(a) => var.variant(5).nest(&a),
});
coarbitrary!(['a] Component<'a>; self, var => match *self {
    Component::Prefix(a) => var.variant(0).nest(&a),
    Component::RootDir => var.variant(1),
    Component::CurDir => var.variant(2),
    Component::ParentDir => var.variant(3),
    Component::Normal(a) => var.variant(4).nest(&a),
});