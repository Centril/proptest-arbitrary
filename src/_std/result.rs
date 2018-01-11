//! Arbitrary implementations for `std::result`.
//! 
use super::*;
use std::result::IntoIter;
use proptest::result;

// We assume that `MaybeOk` is canonical as it's the most likely Strategy
// a user wants.

arbitrary!([A: Arbitrary, B: Arbitrary] Result<A, B>,
    result::MaybeOk<A::Strategy, B::Strategy>,
    product_type![Probability, A::Parameters, B::Parameters];
    args => {
        let product_unpack![prob, a, b] = args;
        let (p, a, b) = (prob.into(), any_with::<A>(a), any_with::<B>(b));
        result::maybe_ok_weighted(p, a, b)
    }
);

impl<A: Debug, E: Arbitrary> functor::ArbitraryF1<A> for Result<A, E>
where
    E::Strategy: 'static
{
    type Parameters = product_type![Probability, E::Parameters];

    fn lift1_with<AS>(base: AS, args: Self::Parameters) -> BoxedStrategy<Self>
    where
        AS: Strategy + 'static,
        AS::Value: ValueTree<Value = A>
    {
        let product_unpack![prob, e] = args;
        let (p, a, e) = (prob.into(), base, any_with::<E>(e));
        result::maybe_ok_weighted(p, a, e).boxed()
    }
}

impl<A: Debug, B: Debug> functor::ArbitraryF2<A, B>
for Result<A, B> {
    type Parameters = Probability;

    fn lift2_with<AS, BS>(fst: AS, snd: BS, args: Self::Parameters)
        -> BoxedStrategy<Self>
    where
        AS: Strategy + 'static,
        AS::Value: ValueTree<Value = A>,
        BS: Strategy + 'static,
        BS::Value: ValueTree<Value = B>
    {
        result::maybe_ok_weighted(args.into(), fst, snd).boxed()
    }
}

arbitrary!([A: Arbitrary] IntoIter<A>,
    SMapped<Result<A, ()>, Self>,
    <Result<A, ()> as Arbitrary>::Parameters;
    args => any_with_smap(args, Result::into_iter)
);

lift1!(['static] IntoIter<A>, Probability; base, args => {
    result::maybe_ok_weighted(args.into(), base, Just(()))
        .prop_map(Result::into_iter)
});

#[cfg(test)]
mod test {
    no_panic_test!(
        result    => Result<u8, u16>,
        into_iter => IntoIter<u8>
    );
}