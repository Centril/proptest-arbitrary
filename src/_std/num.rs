//! Arbitrary implementations for `std::num`.

use super::*;
use std::num::*;

arbitrary!(ParseFloatError; "".parse::<f32>().unwrap_err());
arbitrary!(ParseIntError; "".parse::<u32>().unwrap_err());

#[cfg(feature = "nightly")]
arbitrary!(TryFromIntError; {
    use std::convert::TryFrom;
    u8::try_from(-1).unwrap_err()
});

wrap_ctor!(Wrapping, Wrapping);

arbitrary!(FpCategory,
    TupleUnion<(W<Just<Self>>, W<Just<Self>>, W<Just<Self>>,
                W<Just<Self>>, W<Just<Self>>)>;
    {
        use std::num::FpCategory::*;
        prop_oneof![
            Just(Nan),
            Just(Infinite),
            Just(Zero),
            Just(Subnormal),
            Just(Normal),
        ]
    }
);

#[cfg(test)]
mod test {
    no_panic_test!(
        parse_float_error => ParseFloatError,
        parse_int_error => ParseIntError,
        wrapping => Wrapping<u8>,
        fp_category => FpCategory
    );

    #[cfg(feature = "nightly")]
    no_panic_test!(
        try_from_int_error => TryFromIntError
    );
}