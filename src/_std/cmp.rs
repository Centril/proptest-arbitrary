use super::*;
use std::cmp::{Reverse, Ordering};
use proptest::strategy::Just;
use proptest::strategy::TupleUnion;
use from_mapper::{Mapped, W};

impl<'a, A: Arbitrary<'a>> Arbitrary<'a> for Reverse<A> {
    valuetree!();
    type Parameters = A::Parameters;
    type Strategy = Mapped<'a, A, Self>;
    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        any_with::<A, _>(args).prop_map(Reverse)
    }
}

use self::Ordering::*;

type WJO = W<Just<Ordering>>;

impl_arbitrary!(cmp::Ordering, TupleUnion<(WJO, WJO, WJO)>,
    prop_oneof![Just(Equal), Just(Less), Just(Greater)]
);