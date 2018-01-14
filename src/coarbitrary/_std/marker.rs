use coarbitrary::*;

use std::marker::PhantomData;

impl<T: ?Sized> CoArbitrary for PhantomData<T> {
    fn coarbitrary(&self, _: Perturbable) {}
}