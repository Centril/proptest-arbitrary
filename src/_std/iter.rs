use super::*;
use std::iter::*;
use std::iter::Fuse;

// TODO: Filter, FilterMap, FlatMap, Map, Inspect, Scan, SkipWhile
// Might be possible with CoArbitrary

wrap_ctor!(Once, once);
wrap_ctor!([Iterator<Item = &'a T>, T: 'a + Clone] Cloned, Iterator::cloned);
wrap_ctor!([Iterator + Clone] Cycle, Iterator::cycle);
wrap_ctor!([Iterator] Enumerate, Iterator::enumerate);
wrap_ctor!([Iterator] Fuse, Iterator::fuse);
wrap_ctor!([Iterator<Item = T>, T: Debug] Peekable, Iterator::peekable);
wrap_ctor!([Clone] Repeat, repeat);
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

#[cfg(feature = "nightly")]
usize_mod!(StepBy, step_by);