//! Arbitrary implementations for `std::collections`.

#![cfg_attr(feature="cargo-clippy", allow(implicit_hasher))]

//==============================================================================
// Imports:
//==============================================================================

use super::*;

use std::hash::Hash;
use std::vec::Vec;
use std::rc::Rc;
use std::sync::Arc;
use std::collections::*;

use proptest::collection::*;

//==============================================================================
// Macros:
//==============================================================================

/// Parameters for configuring the generation of `StrategyFor<...<A>>`.
type RangedParams1<A> = product_type![SizeBounds, A];

/// Parameters for configuring the generation of `StrategyFor<...<A, B>>`.
type RangedParams2<A, B> = product_type![SizeBounds, A, B];

macro_rules! impl_1 {
    ($typ: ident, $strat: ident, $($bound : path),* => $fun: ident) => {
        arbitrary_for!([A: Arbitrary<'a> $(+ $bound)*] $typ<A>,
            $strat<A::Strategy>, RangedParams1<A::Parameters>,
            args => {
                let product_unpack![range, a] = args;
                $fun(any_with::<A>(a), range.into())
            });
    };
}

macro_rules! impl_2 {
    ($typ: ident, $strat: ident, $($bound : path),* => $fun: ident) => {
        arbitrary_for!([A: Arbitrary<'a> $(+ $bound)*, B: Arbitrary<'a>]
            $typ<A, B>, $strat<A::Strategy, B::Strategy>,
            RangedParams2<A::Parameters, B::Parameters>,
            args => {
                let product_unpack![range, a, b] = args;
                $fun(any_with::<A>(a), any_with::<B>(b), range.into())
            });
    };
}

//==============================================================================
// Vec, VecDeque, LinkedList, BTreeSet, BinaryHeap, HashSet, HashMap:
//==============================================================================

macro_rules! dst_wrapped {
    ($($w: ident),*) => {
        $(arbitrary_for!([A: Arbitrary<'a>] $w<[A]>,
            FMapped<'a, Vec<A>, Self>, <Vec<A> as Arbitrary<'a>>::Parameters,
            a => any_with_sinto::<Vec<A>, _>(a)
        );)*
    };
}

impl_1!(Vec, VecStrategy, => vec);
dst_wrapped!(Box, Rc, Arc);
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
        let product_unpack![range, a, b] = args;
        btree_map(any_with::<A>(a), any_with::<B>(b), range.into())
    }
}

//==============================================================================
// IntoIterator:
//==============================================================================

macro_rules! into_iter_1 {
    ($module: ident, $type: ident $(, $bound : path)*) => {
        arbitrary_for!([A: Arbitrary<'a> $(+ $bound)*]
            $module::IntoIter<A>,
            SMapped<'a, $type<A>, Self>,
            <$type<A> as Arbitrary<'a>>::Parameters,
            args => any_with_smap(args, $type::into_iter));
    };
}

macro_rules! into_iter_2 {
    ($module: ident, $type: ident $(, $bound : path)*) => {
        arbitrary_for!([A: Arbitrary<'a> $(+ $bound)*, B: Arbitrary<'a>]
            $module::IntoIter<A, B>,
            SMapped<'a, $type<A, B>, Self>,
            <$type<A, B> as Arbitrary<'a>>::Parameters,
            args => any_with_smap(args, $type::into_iter));
    };
}

use std::vec;
into_iter_1!(vec, Vec);
use std::collections::vec_deque;
into_iter_1!(vec_deque, VecDeque);
use std::collections::linked_list;
into_iter_1!(linked_list, LinkedList);
use std::collections::btree_set;
into_iter_1!(btree_set, BTreeSet, Ord);
use std::collections::binary_heap;
into_iter_1!(binary_heap, BinaryHeap, Ord);
use std::collections::hash_set;
into_iter_1!(hash_set, HashSet, Hash, Eq);
use std::collections::hash_map;
into_iter_2!(hash_map, HashMap, Hash, Eq);

use std::collections::btree_map;

impl<'a, A, B> Arbitrary<'a> for btree_map::IntoIter<A, B>
where
    A: Arbitrary<'static> + Ord,
    B: Arbitrary<'static>,
    StrategyFor<A>: 'static,
    StrategyFor<B>: 'static,
{
    valuetree!();
    type Parameters = <BTreeMap<A, B> as Arbitrary<'a>>::Parameters;
    type Strategy = SMapped<'a, BTreeMap<A, B>, Self>;
    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        any_with_smap(args, BTreeMap::into_iter)        
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
            2 => any_with_smap(args.clone(), Bound::Included),
            2 => any_with_smap(args, Bound::Excluded),
            1 => Just(Bound::Unbounded),
        ]
    }
}