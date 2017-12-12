use super::*;
use std::mem::*;

arbitrary_for!([A: Arbitrary<'a>] Discriminant<A>,
    SMapped<'a, A, Self>, A::Parameters,
    args => static_map(any_with::<A>(args), |x| discriminant(&x))
);

// The user is responsible for dropping!
impl_wrap_gen!([] ManuallyDrop);