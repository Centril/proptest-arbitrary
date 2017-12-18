//==============================================================================
// Tuples:
//==============================================================================

use super::*;

macro_rules! impl_tuple {
    ($($typ: ident),*) => {
        impl<'a, $($typ : Arbitrary<'a>),*> Arbitrary<'a> for ($($typ,)*) {
            valuetree!();
            type Parameters = product_type![$($typ::Parameters,)*];
            type Strategy = ($($typ::Strategy,)*);
            fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
                #[allow(non_snake_case)]
                let product_unpack![$($typ),*] = args;
                ($(any_with::<$typ>($typ)),*,)
            }
        }
    };
}

arbitrary!((); ());
impl_tuple!(T0);
impl_tuple!(T0, T1);
impl_tuple!(T0, T1, T2);
impl_tuple!(T0, T1, T2, T3);
impl_tuple!(T0, T1, T2, T3, T4);
impl_tuple!(T0, T1, T2, T3, T4, T5);
impl_tuple!(T0, T1, T2, T3, T4, T5, T6);
impl_tuple!(T0, T1, T2, T3, T4, T5, T6, T7);
impl_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8);
impl_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);

#[cfg(test)]
mod test {
    no_panic_test!(
        tuple_n10 => ((), bool, u8, u16, u32, u64, i8, i16, i32, i64)
    );
}