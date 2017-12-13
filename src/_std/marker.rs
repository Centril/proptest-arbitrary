use std::marker::PhantomData;
arbitrary!([T: ?Sized] PhantomData<T>; PhantomData);