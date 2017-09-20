//==============================================================================
// Primitive types:
//==============================================================================

use super::*;

use proptest::{bool, char};
use proptest::num::{isize, usize, f32, f64, i16, i32, i64, i8, u16, u32, u64, u8};

impls! {
    bool, f32, f64,
    i8, i16, i32, i64, isize,
    u8, u16, u32, u64, usize
}


// TODO: handle this better w.r.t. ParamsType.
impl_arbitrary!(char, char::CharStrategy<'a>, char::ANY);
