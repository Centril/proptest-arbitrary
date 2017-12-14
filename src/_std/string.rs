//==============================================================================
// String:
//==============================================================================

use super::*;
use std::string::{String, FromUtf8Error, FromUtf16Error, ParseError};
use std::iter;
use std::slice;
use std::rc::Rc;
use std::sync::Arc;

/// Wraps the regex that forms the `Strategy` for `String` so that a sensible
/// `Default` can be given. The default is a string of non-control characters.
#[derive(Copy, Clone, From, Into, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StringParam<'a>(&'a str);

impl<'a> Default for StringParam<'a> {
    fn default() -> Self {
        StringParam("\\PC*")
    }
}

impl<'a> Arbitrary<'a> for String {
    valuetree!();
    type Parameters = StringParam<'a>;
    type Strategy = &'a str;

    /// ## Safety
    ///
    /// This implementation panics if the input is not a valid regex proptest
    /// can handle.
    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        args.into()
    }
}

macro_rules! dst_wrapped {
    ($($w: ident),*) => {
        $(arbitrary!($w<str>, FMapped<'a, String, Self>, StringParam<'a>;
            a => any_with_sinto::<String, _>(a)
        );)*
    };
}

dst_wrapped!(Box, Rc, Arc);

generator!(FromUtf16Error, || String::from_utf16(&[0xD800]).unwrap_err());
generator!(ParseError, || panic!());

arbitrary!(FromUtf8Error, SFnPtrMap<BoxedStrategy<Vec<u8>>, Self>;
    static_map(not_utf8_bytes(), |bytes| String::from_utf8(bytes).unwrap_err())
);

pub(crate) fn not_utf8_bytes() -> BoxedStrategy<Vec<u8>> {
    (any::<u16>(), gen_el_bytes()).prop_flat_map(|(valid_up_to, el_bytes)| {
        let bounds: SizeBounds = (valid_up_to as usize).into();
        any_with_map(product_pack![bounds, default()], move |p: Vec<char>| {
            let mut bytes = p.iter().collect::<String>().into_bytes();
            bytes.extend(el_bytes.into_iter());
            bytes
        })
    }).boxed()
}

#[derive(Debug)]
enum ELBytes {
    B1([u8; 1]),
    B2([u8; 2]),
    B3([u8; 3]),
    B4([u8; 4])
}

impl<'a> IntoIterator for &'a ELBytes {
    type Item = u8;
    type IntoIter = iter::Map<slice::Iter<'a, u8>, fn(&u8) -> u8>;
    fn into_iter(self) -> Self::IntoIter {
        use self::ELBytes::*;
        (match *self {
            B1(ref a) => a.iter(),
            B2(ref a) => a.iter(),
            B3(ref a) => a.iter(),
            B4(ref a) => a.iter(),
        }).map(|x| *x)
    }
}

fn b1(a: u8) -> ELBytes { ELBytes::B1([a]) }
fn b2(a: (u8, u8)) -> ELBytes { ELBytes::B2([a.0, a.1]) }
fn b3(a: ((u8, u8), u8)) -> ELBytes { ELBytes::B3([(a.0).0, (a.0).1, a.1]) }
fn b4(a: ((u8, u8), u8, u8)) -> ELBytes {
    ELBytes::B4([(a.0).0, (a.0).1, a.1, a.2])
}

// By analysis of run_utf8_validation defined at:
// https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1429
// we know that .error_len() \in {None, Some(1), Some(2), Some(3)}.
// We represent this with the range [0..4) and generate a valid
// sequence from that.
fn gen_el_bytes() -> BoxedStrategy<ELBytes> {
    /*
    // https://tools.ietf.org/html/rfc3629
    static UTF8_CHAR_WIDTH: [u8; 256] = [
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x1F
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x3F
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x5F
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x7F
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, // 0x9F
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, // 0xBF
    0,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,
    2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2, // 0xDF
    3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3, // 0xEF
    4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0, // 0xFF
    ];
    */
    /*
    /// Mask of the value bits of a continuation byte.
    const CONT_MASK: u8 = 0b0011_1111;
    /// Value of the tag bits (tag mask is !CONT_MASK) of a continuation byte.
    const TAG_CONT_U8: u8 = 0b1000_0000;
    */

    let succ_byte = 0x80u8..0xC0u8;
    let fail_byte = prop_oneof![0x00u8..0x7Fu8, 0xC1u8..];
    let byte0_w0  = prop_oneof![0x80u8..0xC0u8, 0xF5u8..];
    let byte0_w2  = 0xC2u8..0xE0u8;
    let byte0_w3  = 0xE0u8..0xF0u8;
    let byte0_w4  = 0xF0u8..0xF5u8;
    let byte01_w3 = byte0_w3.clone().prop_flat_map(|x| (Just(x), match x {
        0xE0u8          => 0xA0u8..0xC0u8,
        0xE1u8...0xECu8 => 0x80u8..0xC0u8,
        0xEDu8          => 0x80u8..0xA0u8,
        0xEEu8...0xEFu8 => 0x80u8..0xA0u8,
        _               => panic!(),
    }));
    let byte01_w3_e1 = byte0_w3.clone().prop_flat_map(|x| (Just(x), match x {
        0xE0u8          => prop_oneof![..0xA0u8, 0xC0u8..],
        0xE1u8...0xECu8 => prop_oneof![..0x80u8, 0xC0u8..],
        0xEDu8          => prop_oneof![..0x80u8, 0xA0u8..],
        0xEEu8...0xEFu8 => prop_oneof![..0x80u8, 0xA0u8..],
        _               => panic!(),
    }));
    let byte01_w4_e1 = byte0_w4.clone().prop_flat_map(|x| (Just(x), match x {
        0xF0u8          => prop_oneof![..0x90u8, 0xA0u8..],
        0xF1u8...0xF3u8 => prop_oneof![..0x80u8, 0xA0u8..],
        0xF4u8          => prop_oneof![..0x80u8, 0x90u8..],
        _               => panic!()
    }));
    let byte01_w4 = byte0_w4.clone().prop_flat_map(|x| (Just(x), match x {
        0xF0u8          => 0x90u8..0xA0u8,
        0xF1u8...0xF3u8 => 0x80u8..0xA0u8,
        0xF4u8          => 0x80u8..0x90u8,
        _               => panic!()
    }));
    prop_oneof![
        // error_len = None
        prop_oneof![
            // w = 2
            // lacking 1 bytes:
            static_map(byte0_w2.clone(), b1),
            // w = 3
            // lacking 2 bytes:
            static_map(byte0_w3, b1),
            // lacking 1 bytes:
            static_map(byte01_w3.clone(), b2),
            // w = 4
            // lacking 3 bytes:
            static_map(byte0_w4, b1),
            // lacking 2 bytes:
            static_map(byte01_w4.clone(), b2),
            // lacking 1 byte:
            static_map((byte01_w4.clone(), succ_byte.clone()), b3),
        ],
        // error_len = Some(1)
        prop_oneof![
            // w = 1 is not represented.
            // w = 0
            static_map(byte0_w0, b1),
            // w = 2
            static_map((byte0_w2, fail_byte.clone()), b2),
            // w = 3
            static_map(byte01_w3_e1, b2),
            // w = 4
            static_map(byte01_w4_e1, b2),
        ],
        // error_len = Some(2)
        static_map(prop_oneof![
            // w = 3
            (byte01_w3, fail_byte.clone()),
            // w = 4
            (byte01_w4.clone(), fail_byte.clone())
        ], b3),
        // error_len = Some(3), w = 4
        static_map((byte01_w4, succ_byte, fail_byte), b4),
    ].boxed()
}