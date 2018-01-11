//! Arbitrary implementations for `std::iter`.

use super::*;
use std::iter::*;
use std::iter::Fuse;

// TODO: Filter, FilterMap, FlatMap, Map, Inspect, Scan, SkipWhile
// Might be possible with CoArbitrary

wrap_ctor!(Once, once);
wrap_ctor!([Clone] Repeat, repeat);
wrap_ctor!([Iterator + Clone] Cycle, Iterator::cycle);
wrap_ctor!([Iterator] Enumerate, Iterator::enumerate);
wrap_ctor!([Iterator] Fuse, Iterator::fuse);
wrap_ctor!([Iterator<Item = T>, T: Debug] Peekable, Iterator::peekable);
wrap_ctor!([DoubleEndedIterator] Rev, Iterator::rev);

arbitrary!(['a, T: 'a + Clone, A: Arbitrary + Iterator<Item = &'a T>]
    Cloned<A>, SMapped<A, Self>, A::Parameters;
    args => any_with_smap(args, Iterator::cloned));

impl<'a, T: 'static + Clone, A: Debug + 'static + Iterator<Item = &'a T>>
    functor::ArbitraryF1<A>
for Cloned<A> {
    type Parameters = ();

    fn lift1_with<S>(base: S, _args: Self::Parameters) -> BoxedStrategy<Self>
    where
        S: Strategy + 'static,
        S::Value: ValueTree<Value = A>
    {
        base.prop_map(Iterator::cloned).boxed()
    }
}

arbitrary!([A] Empty<A>; empty());

arbitrary!(
    [A: Arbitrary + Iterator, B: Arbitrary + Iterator]
    Zip<A, B>, SMapped<(A, B), Self>,
    product_type![A::Parameters, B::Parameters];
    args => any_with_smap(args, |(a, b)| a.zip(b))
);

lift1!([Debug + 'static + Iterator, B: 'static + Arbitrary + Iterator] Zip<B, A>,
    B::Parameters;
    base, args =>
        (any_with::<B>(args), base).prop_map(|(b, a)| b.zip(a)).boxed()
);

impl<A: Debug + Iterator, B: Debug + Iterator>
    functor::ArbitraryF2<A, B>
for Zip<A, B> {
    type Parameters = ();

    fn lift2_with<AS, BS>(fst: AS, snd: BS, _args: Self::Parameters)
        -> BoxedStrategy<Self>
    where
        AS: Strategy + 'static,
        AS::Value: ValueTree<Value = A>,
        BS: Strategy + 'static,
        BS::Value: ValueTree<Value = B>
    {
        (fst, snd).prop_map(|(a, b)| a.zip(b)).boxed()
    }
}

arbitrary!(
    [T,
     A: Arbitrary + Iterator<Item = T>,
     B: Arbitrary + Iterator<Item = T>]
    Chain<A, B>, SMapped<(A, B), Self>,
    product_type![A::Parameters, B::Parameters];
    args => any_with_smap(args, |(a, b)| a.chain(b))
);

lift1!([Debug + 'static + Iterator<Item = T>,
        B: 'static + Arbitrary + Iterator<Item = T>,
        T]
    Chain<B, A>,
    B::Parameters;
    base, args =>
        (any_with::<B>(args), base).prop_map(|(b, a)| b.chain(a)).boxed()
);

impl<T, A: Debug + Iterator<Item = T>, B: Debug + Iterator<Item = T>>
    functor::ArbitraryF2<A, B>
for Chain<A, B> {
    type Parameters = ();

    fn lift2_with<AS, BS>(fst: AS, snd: BS, _args: Self::Parameters)
        -> BoxedStrategy<Self>
    where
        AS: Strategy + 'static,
        AS::Value: ValueTree<Value = A>,
        BS: Strategy + 'static,
        BS::Value: ValueTree<Value = B>
    {
        (fst, snd).prop_map(|(a, b)| a.chain(b)).boxed()
    }
}

macro_rules! usize_mod {
    ($type: ident, $mapper: ident) => {
        arbitrary!([A: Arbitrary + Iterator] $type<A>,
            SMapped<(A, usize), Self>, A::Parameters;
            a => any_with_smap(product_pack![a, ()], |(a, b)| a.$mapper(b))
        );

        lift1!([Iterator] $type<A>;
            base => (base, any::<usize>()).prop_map(|(a, b)| a.$mapper(b))
        );
    };
}

usize_mod!(Skip, skip);
usize_mod!(Take, take);

#[cfg(feature = "unstable")]
usize_mod!(StepBy, step_by);

#[cfg(test)]
mod test {
    use super::*;

    use std::ops::Range;
    const DUMMY: &'static [u8] = &[0, 1, 2, 3, 4];
    #[derive(Debug)]
    struct Dummy(u8);
    arbitrary!(Dummy, SFnPtrMap<Range<u8>, Self>; static_map(0..5, Dummy));
    impl Iterator for Dummy {
        type Item = &'static u8;
        fn next(&mut self) -> Option<Self::Item> {
            if self.0 < 5 {
                let r = &DUMMY[self.0 as usize];
                self.0 += 1;
                Some(r)
            } else {
                None
            }
        }
    }

    no_panic_test!(
        empty     => Empty<u8>,
        once      => Once<u8>,
        repeat    => Repeat<u8>,
        cloned    => Cloned<super::Dummy>,
        cycle     => Cycle<Once<u8>>,
        enumerate => Enumerate<Repeat<u8>>,
        fuse      => Fuse<Once<u8>>,
        peekable  => Peekable<Repeat<u8>>,
        rev       => Rev<std::vec::IntoIter<u8>>,
        zip       => Zip<Repeat<u8>, Repeat<u16>>,
        chain     => Chain<Once<u8>, Once<u8>>,
        skip      => Skip<Repeat<u8>>,
        take      => Take<Repeat<u8>>
    );

    #[cfg(feature = "unstable")]
    no_panic_test!(
        step_by   => StepBy<Repeat<u8>>
    );
}