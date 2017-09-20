//==============================================================================
// Imports:
//==============================================================================

use super::*;

use std::hash::Hash;
use std::vec::Vec;
use std::collections::*;
use std::ops::Range;

use proptest::collection::*;
use proptest::strategy::{Just, TupleUnion};

use from_mapper::{static_map, StaticMap, W};

//==============================================================================
// Params config structs:
//==============================================================================

/// Parameters for configuring the generation of `StrategyFor<...<A>>`.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct UnaryRangedParams<A> {
    range: Range<usize>,
    a_params: A,
}

/// Parameters for configuring the generation of `StrategyFor<...<A, B>>`.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct BinaryRangedParams<A, B> {
    range: Range<usize>,
    a_params: A,
    b_params: B,
}

impl<A: Default> Default for UnaryRangedParams<A> {
    fn default() -> Self {
        (def(),).into()
    }
}

impl<A: Default> From<()> for UnaryRangedParams<A> {
    fn from(_: ()) -> Self {
        Self::default()
    }
}

impl<A: Default> From<Range<usize>> for UnaryRangedParams<A> {
    fn from(x: Range<usize>) -> Self {
        (x, def()).into()
    }
}

impl<AF, A: From<AF>> From<(AF,)> for UnaryRangedParams<A> {
    fn from(x: (AF,)) -> Self {
        (0..100, x.0).into()
    }
}

impl<AF, A: From<AF>> From<(Range<usize>, AF)> for UnaryRangedParams<A> {
    fn from(x: (Range<usize>, AF)) -> Self {
        Self {
            range: x.0,
            a_params: x.1.into(),
        }
    }
}

impl<A: Default, B: Default> From<()> for BinaryRangedParams<A, B> {
    fn from(_: ()) -> Self {
        Self::default()
    }
}

impl<A: Default, B: Default> Default for BinaryRangedParams<A, B> {
    fn default() -> Self {
        (def(),).into()
    }
}

impl<AF, A: From<AF>, B: Default> From<(AF,)> for BinaryRangedParams<A, B> {
    fn from(x: (AF,)) -> Self {
        (0..100, x.0).into()
    }
}

impl<A: Default, B: Default> From<(Range<usize>)> for BinaryRangedParams<A, B> {
    fn from(x: Range<usize>) -> Self {
        (x, def()).into()
    }
}

impl<AF, A: From<AF>, B: Default> From<(Range<usize>, AF)> for BinaryRangedParams<A, B> {
    fn from(x: (Range<usize>, AF)) -> Self {
        (x.0, x.1, def()).into()
    }
}

impl<AF, A: From<AF>, BF, B: From<BF>> From<(Range<usize>, AF, BF)> for BinaryRangedParams<A, B> {
    fn from(x: (Range<usize>, AF, BF)) -> Self {
        Self {
            range: x.0,
            a_params: x.1.into(),
            b_params: x.2.into(),
        }
    }
}

//==============================================================================
// Vec, VecDeque, LinkedList, BTreeSet, BinaryHeap, HashSet, HashMap:
//==============================================================================

macro_rules! impl_unary {
    ($typ: ident, $strat: ident, $($bound : path),*
        => $fun: ident) => {

        impl<'a, A: Arbitrary<'a> $(+ $bound)*> Arbitrary<'a> for $typ<A> {
            valuetree!();
            type Parameters = UnaryRangedParams<A::Parameters>;
            type Strategy = $strat<A::Strategy>;
            fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
                $fun(arbitrary_with(args.a_params), args.range)
            }
        }
    };
}

macro_rules! impl_binary {
    ($typ: ident, $strat: ident, $($bound : path),* => $fun: ident) => {
        impl<'a, A: Arbitrary<'a> $(+ $bound)* , B: Arbitrary<'a>> Arbitrary<'a>
        for $typ<A, B> {
            valuetree!();
            type Parameters = BinaryRangedParams<A::Parameters, B::Parameters>;
            type Strategy = $strat<A::Strategy, B::Strategy>;
            fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
                $fun(arbitrary_with(args.a_params),
                     arbitrary_with(args.b_params),
                     args.range)
            }
        }
    };
}

impl_unary!(Vec, VecStrategy, => vec);
impl_unary!(VecDeque, VecDequeStrategy, => vec_deque);
impl_unary!(LinkedList, LinkedListStrategy, => linked_list);
impl_unary!(BTreeSet, BTreeSetStrategy, Ord => btree_set);
impl_unary!(BinaryHeap, BinaryHeapStrategy, Ord => binary_heap);
impl_unary!(HashSet, HashSetStrategy, Hash, Eq => hash_set);
impl_binary!(HashMap, HashMapStrategy, Hash, Eq => hash_map);

//==============================================================================
// BTreeMap:
//==============================================================================

impl<'a, A, B> Arbitrary<'a> for BTreeMap<A, B>
where
    A: Arbitrary<'static> + Ord,
    B: Arbitrary<'static>,
    StrategyFor<A>: 'static,
    StrategyFor<B>: 'static,
{
    valuetree!();
    type Parameters = BinaryRangedParams<A::Parameters, B::Parameters>;
    type Strategy = BTreeMapStrategy<A::Strategy, B::Strategy>;
    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        btree_map(
            arbitrary_with(args.a_params),
            arbitrary_with(args.b_params),
            args.range,
        )
    }
}

//==============================================================================
// Bound:
//==============================================================================

type BoundFn<A> = fn(A) -> Bound<A>;
type SM<'a, A> = StaticMap<<A as Arbitrary<'a>>::Strategy, A, Bound<A>, BoundFn<A>>;
type BoundStrategy<'a, A> = TupleUnion<(W<SM<'a, A>>, W<SM<'a, A>>, W<Just<Bound<A>>>)>;

impl<'a, A: Arbitrary<'a> + Clone> Arbitrary<'a> for Bound<A>
where
    A::Parameters: Clone,
{
    valuetree!();
    type Parameters = A::Parameters;
    type Strategy = BoundStrategy<'a, A>;
    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        prop_oneof![
            2 => static_map(any_with::<A, _>(args.clone()), Bound::Included),
            2 => static_map(any_with::<A, _>(args), Bound::Excluded),
            1 => Just(Bound::Unbounded),
        ]
    }
}

//==============================================================================
// Alloc, i.e: Box, ...:
//==============================================================================
