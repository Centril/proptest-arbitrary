//! Arbitrary implementations for `std::mem`.

use super::*;
use std::mem::*;

arbitrary!([A: Arbitrary] Discriminant<A>,
    SMapped<A, Self>, A::Parameters;
    args => static_map(any_with::<A>(args), |x| discriminant(&x))
);

lift1!(['static] Discriminant<A>;
    base => static_map(base, |x| discriminant(&x))
);

// The user is responsible for dropping!
wrap_ctor!(ManuallyDrop);

#[cfg(test)]
mod test {
    #[derive(Copy, Clone, Debug)]
    struct DummyStruct;
    arbitrary!(DummyStruct; DummyStruct);

    no_panic_test!(
        discriminant_struct => Discriminant<super::DummyStruct>,
        discriminant_enum   => Discriminant<std::num::FpCategory>,
        manually_drop       => ManuallyDrop<u8> // Trivial destructor.
    );
}