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

impl<'a> Arbitrary<'a> for ffi::FromBytesWithNulError {
    valuetree!();
    params!();
    type Strategy = Mapped<'a, Option<u16>, Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        any::<Option<u16>>().prop_map(|opt_pos| {
            // We make some assumptions about the internal structure of
            // ffi::FromBytesWithNulError. However, these assumptions do not
            // involve any non-public API.
            if let Some(pos) = opt_pos {
                let pos = pos as usize;
                // Allocate pos + 2 so that we never reallocate:
                let mut v = Vec::<u8>::with_capacity(pos + 2);
                v.extend(::std::iter::repeat(1).take(pos));
                v.push(0);
                v.push(1);
                ffi::CStr::from_bytes_with_nul(v.as_slice()).unwrap_err()
            } else {
                ffi::CStr::from_bytes_with_nul(b"").unwrap_err()
            }
        })
    }
}

/*
IntoStringError:

1. Generate triple (valid_up_to: usize, error_len: Option<usize>, suffix: usize)
2. Compute error from this.
3. Algorithm is same for Utf8Error.
*/