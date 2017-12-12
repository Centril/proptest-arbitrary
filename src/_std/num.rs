use super::*;
use std::num::*;
use std::num::FpCategory::*;

impl_just!(ParseFloatError, "".parse::<f32>().unwrap_err());
impl_just!(ParseIntError, "".parse::<u32>().unwrap_err());
impl_wrap_gen!([] Wrapping, Wrapping);

#[cfg(feature = "nightly")]
impl_just!(TryFromIntError, {
    use std::convert::TryFrom;
    u8::try_from(-1).unwrap_err()
});

impl_arbitrary!(FpCategory,
    TupleUnion<(W<Just<Self>>, W<Just<Self>>, W<Just<Self>>,
                W<Just<Self>>, W<Just<Self>>)>,
    prop_oneof![
        Just(Nan),
        Just(Infinite),
        Just(Zero),
        Just(Subnormal),
        Just(Normal),
    ]
);