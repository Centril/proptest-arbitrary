use std::rc::Rc;
impl_wrap_from!([] Rc);
// Weak would always give None on upgrade since there's no owned Rc.