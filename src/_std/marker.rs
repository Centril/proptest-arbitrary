use super::*;
use std::marker::PhantomData;

arbitrary_for!([T: ?Sized] PhantomData<T>, Just<Self>, (),
    _a => Just(PhantomData)
);