use std::marker::PhantomData;
use proptest::strategy::Just;

arbitrary_for!([T: ?Sized] PhantomData<T>, Just<Self>, (),
    _a => Just(PhantomData)
);