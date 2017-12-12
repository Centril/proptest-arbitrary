//! Arbitrary for `std::cell`.

use std::cell::{Cell, RefCell, UnsafeCell, BorrowError, BorrowMutError};

impl_wrap_from!([Copy] Cell);
impl_wrap_from!([] RefCell);
impl_wrap_from!([] UnsafeCell);

gen_strat!(BorrowError, || {
    // False positive:
    #[cfg_attr(feature = "cargo-clippy", allow(let_and_return))]
    {
        let _rc = RefCell::new(());
        let _bm = _rc.borrow_mut();
        let _tb = _rc.try_borrow();
        let ret = match _rc.try_borrow() {
            Ok(_) => panic!("should never happen!"),
            Err(e) => e,
        };
        ret
    }
});
gen_strat!(BorrowMutError, || {
    // False positive:
    #[cfg_attr(feature = "cargo-clippy", allow(let_and_return))]
    {
        let _rc = RefCell::new(());
        let _bm = _rc.borrow_mut();
        let _tb = _rc.try_borrow();
        let ret = match _rc.try_borrow_mut() {
            Ok(_) => panic!("should never happen!"),
            Err(e) => e,
        };
        ret
    }
});