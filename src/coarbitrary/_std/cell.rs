use coarbitrary::*;

use std::cell::*;

coarbitrary!([A: CoArbitrary + Copy] Cell<A>;
    self, var => var.nest(&self.get()));

delegate_deref!(['b, A: CoArbitrary + ?Sized] Ref<'b, A>);
delegate_deref!(['b, A: CoArbitrary + ?Sized] RefMut<'b, A>);

impl<A: CoArbitrary + ?Sized> CoArbitrary for RefCell<A> {
    /// Perturbs the given underlying RNG according to
    /// the structure of the given `&self` value.
    /// 
    /// # Safety
    ///
    /// We use the same mechanics as `PartialEq` and similar
    /// trait impls for `RefCell`. Those impls use `.borrow()`
    /// internally. If the `RefCell` is already mutably borrowed,
    /// calling this will panic.
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.borrow());
    }
}

coarbitrary_unit!(BorrowError, BorrowMutError);

/*

// We could provide:
//
// But it is not clear that this is a reasonable definition of CoArbitrary.
// It may be too strong since it relies on pointer equality and not
// structural equality.

impl<A> CoArbitrary for UnsafeCell<A> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&(self.get() as usize));
    }
}

*/