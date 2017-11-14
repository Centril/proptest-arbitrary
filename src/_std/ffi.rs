use super::*;
use std::ffi;
use std::ops::Range;
use from_mapper::{static_map, SFnPtrMap};
use proptest::collection::{VecStrategy, vec};

impl<'a> Arbitrary<'a> for ffi::CString {
    valuetree!();
    type Parameters = CollectionSizeBounds;
    type Strategy = SFnPtrMap<VecStrategy<Range<u8>>, Self>;
    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        static_map(vec(1..std::u8::MAX, (args + 1).into()), |mut vec| {
            vec.pop().unwrap();
            unsafe {
                Self::from_vec_unchecked(vec)
            }
        })
    }
}

impl<'a> Arbitrary<'a> for Box<ffi::CStr> {
    valuetree!();
    type Parameters = CollectionSizeBounds;
    type Strategy = Mapped<'a, ffi::CString, Self>;
    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        any_with::<ffi::CString, _>(args)
            .prop_map(ffi::CString::into_boxed_c_str)
    }
}