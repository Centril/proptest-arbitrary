//! Arbitrary for `std::cell`.

use super::*;
use extras::GenStrategy;

use std::cell::{Cell, RefCell, UnsafeCell, BorrowError, BorrowMutError};

impl_wrap_gen!(Cell, Copy);
impl_wrap_gen!(RefCell);
impl_wrap_gen!(UnsafeCell);

impl_arbitrary!(BorrowError, GenStrategy<Self>, {
    fn borrow_error() -> BorrowError {
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
    }
    GenStrategy::new(borrow_error)
});

impl_arbitrary!(BorrowMutError, GenStrategy<Self>, {
    fn borrow_error() -> BorrowMutError {    
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
    }
    GenStrategy::new(borrow_error)
});