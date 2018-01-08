//! Arbitrary implementations for arrays.

use super::*;

use proptest::array::UniformArrayStrategy;

macro_rules! array {
    ($($n: expr),*) => { $(
        impl<'a, A: Arbitrary<'a>> Arbitrary<'a> for [A; $n] {
            valuetree!();
            type Parameters = A::Parameters;
            type Strategy = UniformArrayStrategy<A::Strategy, [A; $n]>;
            fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
                let base = any_with::<A>(args);
                UniformArrayStrategy::new(base)
            }
        }
    )* };
}

array!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
       21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32);

#[cfg(test)]
mod test {
    no_panic_test!(
        array_32 => [u8; 32]
    );
}