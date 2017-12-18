//! Arbitrary implementations for `std::ffi`.

use super::*;
use std::ffi::*;
use std::ops::Range;
use proptest::collection::{VecStrategy, vec};
use _std::string::not_utf8_bytes;

arbitrary!(CString,
    SFnPtrMap<VecStrategy<Range<u8>>, Self>, SizeBounds;
    args => static_map(vec(1..std::u8::MAX, (args + 1).into()), |mut vec| {
        vec.pop().unwrap();
        // Could use: Self::from_vec_unchecked(vec) safely.
        Self::new(vec).unwrap()
    })
);

arbitrary!(OsString, FMapped<'a, String, Self>,
    <String as Arbitrary<'a>>::Parameters; a => any_with_sinto::<String, _>(a)
);

macro_rules! dst_wrapped {
    ($($w: ident),*) => {
        $(arbitrary!($w<CStr>, FMapped<'a, CString, Self>, SizeBounds;
            a => any_with_sinto::<CString, _>(a)
        );)*
        $(arbitrary!($w<OsStr>, FMapped<'a, OsString, Self>,
            <String as Arbitrary<'a>>::Parameters;
            a => any_with_sinto::<OsString, _>(a)
        );)*
    };
}

dst_wrapped!(Box);

#[cfg(MIN_VER_1_24_0)]
use std::rc::Rc;
#[cfg(MIN_VER_1_24_0)]
use std::sync::Arc;
#[cfg(MIN_VER_1_24_0)]
dst_wrapped!(Rc, Arc);

arbitrary!(FromBytesWithNulError, SMapped<'a, Option<u16>, Self>; {
    static_map(any::<Option<u16>>(), |opt_pos| {
        // We make some assumptions about the internal structure of
        // FromBytesWithNulError. However, these assumptions do not
        // involve any non-public API.
        if let Some(pos) = opt_pos {
            let pos = pos as usize;
            // Allocate pos + 2 so that we never reallocate:
            let mut v = Vec::<u8>::with_capacity(pos + 2);
            v.extend(::std::iter::repeat(1).take(pos));
            v.push(0);
            v.push(1);
            CStr::from_bytes_with_nul(v.as_slice()).unwrap_err()
        } else {
            CStr::from_bytes_with_nul(b"").unwrap_err()
        }
    })
});

arbitrary!(IntoStringError, SFnPtrMap<BoxedStrategy<Vec<u8>>, Self>;
    static_map(not_utf8_bytes(), |bytes|
        CString::new(bytes).unwrap().into_string().unwrap_err()
    )
);

#[cfg(test)]
mod test {
    no_panic_test!(
        c_string => CString,
        os_string => OsString,
        box_c_str => Box<CStr>,
        box_os_str => Box<OsStr>,
        from_bytes_with_nul => FromBytesWithNulError
        //TODO
        //into_string_error => IntoStringError
    );
    #[cfg(MIN_VER_1_24_0)]
    no_panic_test!(
        rc_c_str => Rc<CStr>,
        rc_os_str => Rc<OsStr>,
        arc_c_str => Arc<CStr>,
        arc_os_str => Arc<OsStr>
    );
}