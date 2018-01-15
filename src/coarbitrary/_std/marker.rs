use std::marker::PhantomData;

coarbitrary!([T: ?Sized] PhantomData<T>; self, _var => {});