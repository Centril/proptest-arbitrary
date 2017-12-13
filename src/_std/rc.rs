use std::rc::Rc;

// Weak would always give None on upgrade since there's no owned Rc.

wrap_from!(Rc);