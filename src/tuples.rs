//==============================================================================
// Tuples:
//==============================================================================

use super::*;

macro_rules! impl_tuple {
    ($($typ: ident),*) => {
        impl<'a, $($typ : Arbitrary<'a>),*> Arbitrary<'a> for ($($typ,)*) {
            valuetree!();
            type Strategy = ($($typ::Strategy,)*);
            fn arbitrary() -> Self::Strategy {
                ($(any::<$typ>()),*,)
            }
        }
    };
}

impl_tuple!(A);
impl_tuple!(A, B);
impl_tuple!(A, B, C);
impl_tuple!(A, B, C, D);
impl_tuple!(A, B, C, D, E);
impl_tuple!(A, B, C, D, E, F);
impl_tuple!(A, B, C, D, E, F, G);
impl_tuple!(A, B, C, D, E, F, G, H);
impl_tuple!(A, B, C, D, E, F, G, H, I);
impl_tuple!(A, B, C, D, E, F, G, H, I, J);