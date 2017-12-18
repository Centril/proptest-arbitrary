//! Arbitrary implementations for `std::iter`.

use super::*;
use std::iter::*;
use std::iter::Fuse;

// TODO: Filter, FilterMap, FlatMap, Map, Inspect, Scan, SkipWhile
// Might be possible with CoArbitrary

wrap_ctor!(Once, once);
wrap_ctor!([Clone] Repeat, repeat);
wrap_ctor!([Iterator<Item = &'a T>, T: 'a + Clone] Cloned, Iterator::cloned);
wrap_ctor!([Iterator + Clone] Cycle, Iterator::cycle);
wrap_ctor!([Iterator] Enumerate, Iterator::enumerate);
wrap_ctor!([Iterator] Fuse, Iterator::fuse);
wrap_ctor!([Iterator<Item = T>, T: Debug] Peekable, Iterator::peekable);
wrap_ctor!([DoubleEndedIterator] Rev, Iterator::rev);

arbitrary!([A] Empty<A>; empty());

arbitrary!(
    [A: Arbitrary<'a> + Iterator, B: Arbitrary<'a> + Iterator]
    Zip<A, B>, SMapped<'a, (A, B), Self>,
    product_type![A::Parameters, B::Parameters];
    args => any_with_smap(args, |(a, b)| a.zip(b))
);

arbitrary!(
    [T,
     A: Arbitrary<'a> + Iterator<Item = T>,
     B: Arbitrary<'a> + Iterator<Item = T>]
    Chain<A, B>, SMapped<'a, (A, B), Self>,
    product_type![A::Parameters, B::Parameters];
    args => any_with_smap(args, |(a, b)| a.chain(b))
);

macro_rules! usize_mod {
    ($type: ident, $mapper: ident) => {
        arbitrary!([A: Arbitrary<'a> + Iterator] $type<A>,
            SMapped<'a, (A, usize), Self>, A::Parameters;
            a => any_with_smap(product_pack![a, ()], |(a, b)| a.$mapper(b))
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