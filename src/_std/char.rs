use super::*;
use std::char;
use proptest::strategy::{Just, IndFlatten, Flatten};
use from_mapper::{Mapped, SMapped, static_map};

macro_rules! impl_wrap_char {
    ($type: ty, $mapper: expr) => {
        impl<'a> Arbitrary<'a> for $type {
            valuetree!();
            type Parameters = ParamsType<'a, char>;
            type Strategy = SMapped<'a, char, Self>;

            fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
                static_map(arbitrary_with(args), $mapper)
            }
        }
    };
}

// TODO: DecodeUtf16, DecodeUtf16Error  must de Debug.

impl_wrap_char!(char::EscapeDebug, char::escape_debug);
impl_wrap_char!(char::EscapeDefault, char::escape_default);
impl_wrap_char!(char::EscapeUnicode, char::escape_unicode);
//impl_wrap_char!(char::ToLowercase, char::to_lowercase); // TODO: Debug
//impl_wrap_char!(char::ToUppercase, char::to_uppercase); // TODO: Debug

impl<'a> Arbitrary<'a> for char::ParseCharError {
    valuetree!();
    params!();
    type Strategy = IndFlatten<Mapped<'a, bool, Just<Self>>>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        any::<bool>().prop_ind_flat_map(|is_two|
            Just((if is_two { "__" } else { "" }).parse::<char>().unwrap_err())
        )
    }
}

impl<'a> Arbitrary<'a> for char::CharTryFromError {
    valuetree!();
    params!();
    type Strategy = Just<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use std::convert::TryFrom;
        Just(char::try_from(0xD800 as u32).unwrap_err())
    }
}

impl<'a> Arbitrary<'a>
for char::DecodeUtf8<<Vec<u8> as IntoIterator>::IntoIter> {
    valuetree!();
    params!();
    type Strategy = Flatten<Mapped<'a, u16, Mapped<'a, Vec<u8>, Self>>>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        any::<u16>().prop_flat_map(|size|
            any_with::<Vec<u8>, _>(
                hlist![CollectionSizeBounds::new(0..(size as usize)), ()]
            )
            .prop_map(char::decode_utf8)
        )
    }
}