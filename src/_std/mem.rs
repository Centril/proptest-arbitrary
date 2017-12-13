use super::*;
use std::mem::*;

arbitrary!([A: Arbitrary<'a>] Discriminant<A>,
    SMapped<'a, A, Self>, A::Parameters;
    args => static_map(any_with::<A>(args), |x| discriminant(&x))
);

// The user is responsible for dropping!
wrap_ctor!(ManuallyDrop);