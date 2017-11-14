#![cfg_attr(feature="cargo-clippy", allow(implicit_hasher))]

//==============================================================================
// Imports:
//==============================================================================

use std::hash::Hash;
use std::vec::Vec;
use std::collections::*;

use super::*;
use ::from_mapper::{W, static_map, SMapped};

use proptest::collection::*;
use proptest::strategy::{Just, TupleUnion};

/// Parameters for configuring the generation of `StrategyFor<...<A>>`.
type RangedParams1<A> = Hlist![CollectionSizeBounds, A];

/// Parameters for configuring the generation of `StrategyFor<...<A, B>>`.
type RangedParams2<A, B> = Hlist![CollectionSizeBounds, A, B];

macro_rules! impl_1 {
    ($typ: ident, $strat: ident, $($bound : path),*
        => $fun: ident) => {
        impl<'a, A: Arbitrary<'a> $(+ $bound)*> Arbitrary<'a> for $typ<A> {
            valuetree!();
            type Parameters = RangedParams1<A::Parameters>;
            type Strategy = $strat<A::Strategy>;
            fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
                let hlist_pat![range, a] = args;
                $fun(arbitrary_with(a), range.into())
            }
        }
    };
}

macro_rules! impl_2 {
    ($typ: ident, $strat: ident, $($bound : path),* => $fun: ident) => {
        impl<'a, A: Arbitrary<'a> $(+ $bound)* , B: Arbitrary<'a>> Arbitrary<'a>
        for $typ<A, B> {
            valuetree!();
            type Parameters = RangedParams2<A::Parameters, B::Parameters>;
            type Strategy = $strat<A::Strategy, B::Strategy>;
            fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
                let hlist_pat![range, a, b] = args;
                $fun(arbitrary_with(a), arbitrary_with(b), range.into())
            }
        }
    };
}

//==============================================================================
// Vec, VecDeque, LinkedList, BTreeSet, BinaryHeap, HashSet, HashMap:
//==============================================================================

impl_1!(Vec, VecStrategy, => vec);
impl_1!(VecDeque, VecDequeStrategy, => vec_deque);
impl_1!(LinkedList, LinkedListStrategy, => linked_list);
impl_1!(BTreeSet, BTreeSetStrategy, Ord => btree_set);
impl_1!(BinaryHeap, BinaryHeapStrategy, Ord => binary_heap);
impl_1!(HashSet, HashSetStrategy, Hash, Eq => hash_set);
impl_2!(HashMap, HashMapStrategy, Hash, Eq => hash_map);

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
    type Parameters = RangedParams2<A::Parameters, B::Parameters>;
    type Strategy = BTreeMapStrategy<A::Strategy, B::Strategy>;
    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        let hlist_pat![range, a, b] = args;
        btree_map(arbitrary_with(a), arbitrary_with(b), range.into())
    }
}

//==============================================================================
// Bound:
//==============================================================================

impl<'a, A: Arbitrary<'a> + Clone> Arbitrary<'a> for Bound<A>
where
    ParamsType<'a, A>: Clone,
{
    valuetree!();
    type Parameters = A::Parameters;
    type Strategy =
        TupleUnion<(
            W<SMapped<'a, A, Self>>,
            W<SMapped<'a, A, Self>>,
            W<Just<Self>>
        )>;

    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        prop_oneof![
            2 => static_map(any_with::<A, _>(args.clone()), Bound::Included),
            2 => static_map(any_with::<A, _>(args), Bound::Excluded),
            1 => Just(Bound::Unbounded),
        ]
    }
}