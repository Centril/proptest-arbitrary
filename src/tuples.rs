//==============================================================================
// Tuples:
//==============================================================================

use super::*;
use proptest::strategy::Just;

macro_rules! impl_tuple {
    ($($typ: ident),*) => {
        impl<'a, $($typ : Arbitrary<'a>),*> Arbitrary<'a> for ($($typ,)*) {
            valuetree!();
            type Parameters = Hlist![$($typ::Parameters,)*];
            type Strategy = ($($typ::Strategy,)*);
            fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
                #[allow(non_snake_case)]
                let hlist_pat![$($typ),*] = args;
                ($(any_with::<$typ>($typ)),*,)
            }
        }
    };
}

impl_arbitrary!((), Just<()>, Just(()));

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