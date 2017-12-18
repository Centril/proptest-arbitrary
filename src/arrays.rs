//! Arbitrary implementations for arrays.

use super::*;

macro_rules! small_array {
    ($n:tt : $($ix:expr),*) => {
        impl<'a, A: Arbitrary<'a>> Arbitrary<'a> for [A; $n]
        where
            ParamsType<'a, A>: Clone
        {
            valuetree!();
            type Parameters = A::Parameters;
            type Strategy = [A::Strategy; $n];
            fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {                
                [
                    $({
                        let _ = $ix;
                        any_with::<A>(args.clone())
                    }),*
                ]
            }
        }
    };
}

small_array!(1: 0);
small_array!(2: 0, 1);
small_array!(3: 0, 1, 2);
small_array!(4: 0, 1, 2, 3);
small_array!(5: 0, 1, 2, 3, 4);
small_array!(6: 0, 1, 2, 3, 4, 5);
small_array!(7: 0, 1, 2, 3, 4, 5, 6);
small_array!(8: 0, 1, 2, 3, 4, 5, 6, 7);
small_array!(9: 0, 1, 2, 3, 4, 5, 6, 7, 8);
small_array!(10: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
small_array!(11: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
small_array!(12: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
small_array!(13: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
small_array!(14: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
small_array!(15: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14);
small_array!(16: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
small_array!(17: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
small_array!(18: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17);
small_array!(19: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
                 18);
small_array!(20: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
                 18, 19);
small_array!(21: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
                 18, 19, 20);
small_array!(22: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
                 18, 19, 20, 21);
small_array!(23: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
                 18, 19, 20, 21, 22);
small_array!(24: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
                 18, 19, 20, 21, 22, 23);
small_array!(25: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
                 18, 19, 20, 21, 22, 23, 24);
small_array!(26: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
                 18, 19, 20, 21, 22, 23, 24, 25);
small_array!(27: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
                 18, 19, 20, 21, 22, 23, 24, 25, 26);
small_array!(28: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
                 18, 19, 20, 21, 22, 23, 24, 25, 26, 27);
small_array!(29: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
                 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28);
small_array!(30: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
                 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29);
small_array!(31: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
                 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30);
small_array!(32: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
                 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31);

#[cfg(test)]
mod test {
    no_panic_test!(
        array_32 => [u8; 32]
    );
}