use super::*;
use std::iter::*;
use std::iter::Fuse;
use frunk_core::hlist::LiftInto;

// TODO: Filter, FilterMap, FlatMap, Map, Inspect, Scan, SkipWhile
// Might be possible with CoArbitrary

impl_wrap_gen!([Iterator<Item = &'a T>, T: 'a + Clone] Cloned, Iterator::cloned);
impl_wrap_gen!([Iterator + Clone] Cycle, Iterator::cycle);
impl_wrap_gen!([Iterator] Enumerate, Iterator::enumerate);
impl_wrap_gen!([Iterator] Fuse, Iterator::fuse);
impl_wrap_gen!([] Once, once);
impl_wrap_gen!([Iterator<Item = T>, T: Debug] Peekable, Iterator::peekable);
impl_wrap_gen!([Clone] Repeat, repeat);
impl_wrap_gen!([DoubleEndedIterator] Rev, Iterator::rev);

arbitrary_for!([A] Empty<A>, Just<Self>, (), _a => Just(empty()));

arbitrary_for!(
    [A: Arbitrary<'a> + Iterator, B: Arbitrary<'a> + Iterator]
    Zip<A, B>,
    SMapped<'a, (A, B), Self>,
    Hlist![A::Parameters, B::Parameters],
    args => any_with_smap(args, |(a, b)| a.zip(b))
);
arbitrary_for!(
    [T,
     A: Arbitrary<'a> + Iterator<Item = T>,
     B: Arbitrary<'a> + Iterator<Item = T>]
    Chain<A, B>,
    SMapped<'a, (A, B), Self>,
    Hlist![A::Parameters, B::Parameters],
    args => any_with_smap(args, |(a, b)| a.chain(b))
);

macro_rules! usize_mod {
    ($type: ident, $mapper: ident) => {
        arbitrary_for!(
            [A: Arbitrary<'a> + Iterator]
            $type<A>,
            SMapped<'a, (A, usize), Self>,
            A::Parameters,
            args => any_with_smap(args.lift_into(), |(a, b)| a.$mapper(b))
        );
    };
}

usize_mod!(Skip, skip);
usize_mod!(Take, take);

#[cfg(feature = "nightly")]
usize_mod!(StepBy, step_by);