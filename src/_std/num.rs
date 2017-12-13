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