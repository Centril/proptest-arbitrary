//! Arbitrary for `std::cell`.

use std::cell::{Cell, RefCell, UnsafeCell, BorrowError, BorrowMutError};

wrap_from!([Copy] Cell);
wrap_from!(RefCell);
wrap_from!(UnsafeCell);

generator!(BorrowError, || {
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
generator!(BorrowMutError, || {
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