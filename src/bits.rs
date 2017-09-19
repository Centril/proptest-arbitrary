//==============================================================================
// Bits:
//==============================================================================

use super::*;
use from_mapper::*;

use proptest::bits::{self, BitSetLike, BitSetStrategy};

/// Bits is a simple newtype for treating the generic type parameter `T` as
/// a set of bits for the purposes of production of arbitrary values.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Bits<T: BitSetLike>(T);

impl<T: BitSetLike> From<T> for Bits<T> {
    fn from(x: T) -> Self {
        Bits(x)
    }
}

macro_rules! impl_bits {
    ($([$typ: ty => $arb_strat_val: expr]),*) => {
        $(
            impl From<Bits<Self>> for $typ {
                fn from(x: Bits<Self>) -> Self { x.0 }
            }

            impl<'a> Arbitrary<'a> for Bits<$typ> {
                valuetree!();
                type Strategy = FromMapStrategy<BitSetStrategy<$typ>, $typ, Self>;
                fn arbitrary() -> Self::Strategy {
                    FromMapStrategy::new($arb_strat_val, FromMapper::default())
                }
            }
        )*
    };
}

impl_bits!([i8 => bits::i8::ANY],
           [i16 => bits::i16::ANY],
           [i32 => bits::i32::ANY],
           [i64 => bits::i64::ANY],
           [isize => bits::isize::masked(!0isize)],
           [u8 =>  bits::u8::ANY],
           [u16 => bits::u16::ANY],
           [u32 => bits::u32::ANY],
           [u64 => bits::u64::ANY],
           [usize => bits::usize::masked(!0usize)]);