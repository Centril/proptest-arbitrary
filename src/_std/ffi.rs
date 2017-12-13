use super::*;
use std::ffi;
use std::ops::Range;
use std::rc::Rc;
use std::sync::Arc;
use proptest::collection::{VecStrategy, vec};
use _std::string::not_utf8_bytes;

arbitrary!(ffi::CString,
    SFnPtrMap<VecStrategy<Range<u8>>, Self>, SizeBounds;
    args => static_map(vec(1..std::u8::MAX, (args + 1).into()), |mut vec| {
        vec.pop().unwrap();
        // Could use: Self::from_vec_unchecked(vec) safely.
        Self::new(vec).unwrap()
    })
);

arbitrary!(ffi::OsString, FMapped<'a, String, Self>,
    <String as Arbitrary<'a>>::Parameters; a => any_with_sinto::<String, _>(a)
);

macro_rules! dst_wrapped {
    ($($w: ident),*) => {
        $(arbitrary!($w<ffi::CStr>,
            FMapped<'a, ffi::CString, Self>, SizeBounds;
            a => any_with_sinto::<ffi::CString, _>(a)
        );)*
        $(arbitrary!($w<ffi::OsStr>, FMapped<'a, ffi::OsString, Self>,
            <String as Arbitrary<'a>>::Parameters;
            a => any_with_sinto::<ffi::OsString, _>(a)
        );)*
    };
}

dst_wrapped!(Box);

#[cfg(MIN_VER_1_24_0)]
dst_wrapped!(Rc, Arc);

arbitrary!(ffi::FromBytesWithNulError, SMapped<'a, Option<u16>, Self>; {
    static_map(any::<Option<u16>>(), |opt_pos| {
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
});

arbitrary!(ffi::IntoStringError, SFnPtrMap<BoxedStrategy<Vec<u8>>, Self>;
    static_map(not_utf8_bytes(), |bytes|
        ffi::CString::new(bytes).unwrap().into_string().unwrap_err()
    )
);