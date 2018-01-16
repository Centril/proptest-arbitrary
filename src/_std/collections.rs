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
use std::vec;

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
        arbitrary!([A: Arbitrary $(+ $bound)*] $typ<A>,
            $strat<A::Strategy>, RangedParams1<A::Parameters>;
            args => {
                let product_unpack![range, a] = args;
                $fun(any_with::<A>(a), range.into())
            });

        lift1!([$($bound+)*] $typ<A>, SizeBounds;
            base, args => $fun(base, args.into()));
    };
}

//==============================================================================
// Vec, VecDeque, LinkedList, BTreeSet, BinaryHeap, HashSet, HashMap:
//==============================================================================

macro_rules! dst_wrapped {
    ($($w: ident),*) => {
        $(arbitrary!([A: Arbitrary] $w<[A]>,
            FMapped<Vec<A>, Self>, <Vec<A> as Arbitrary>::Parameters;
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

//==============================================================================
// IntoIterator:
//==============================================================================

macro_rules! into_iter_1 {
    ($module: ident, $type: ident $(, $bound : path)*) => {
        arbitrary!([A: Arbitrary $(+ $bound)*]
            $module::IntoIter<A>,
            SMapped<$type<A>, Self>,
            <$type<A> as Arbitrary>::Parameters;
            args => any_with_smap(args, $type::into_iter));

        lift1!(['static + $($bound+)*] $module::IntoIter<A>, SizeBounds;
            base, args =>
                $module(base, args.into()).prop_map($type::into_iter));
    };
}

into_iter_1!(vec, Vec);
into_iter_1!(vec_deque, VecDeque);
into_iter_1!(linked_list, LinkedList);
into_iter_1!(btree_set, BTreeSet, Ord);
into_iter_1!(binary_heap, BinaryHeap, Ord);
into_iter_1!(hash_set, HashSet, Hash, Eq);

//==============================================================================
// HashMap:
//==============================================================================

arbitrary!([A: Arbitrary + Hash + Eq, B: Arbitrary] HashMap<A, B>,
    HashMapStrategy<A::Strategy, B::Strategy>,
    RangedParams2<A::Parameters, B::Parameters>;
    args => {
        let product_unpack![range, a, b] = args;
        hash_map(any_with::<A>(a), any_with::<B>(b), range.into())
    });

arbitrary!([A: Arbitrary + Hash + Eq, B: Arbitrary] hash_map::IntoIter<A, B>,
    SMapped<HashMap<A, B>, Self>,
    <HashMap<A, B> as Arbitrary>::Parameters;
    args => any_with_smap(args, HashMap::into_iter));

lift1!([, K: Hash + Eq + Arbitrary + 'static] HashMap<K, A>,
    RangedParams1<K::Parameters>;
    base, args => {
        let product_unpack![range, k] = args;
        hash_map(any_with::<K>(k), base, range.into())
    }
);

lift1!(['static, K: Hash + Eq + Arbitrary + 'static] hash_map::IntoIter<K, A>,
    RangedParams1<K::Parameters>;
    base, args => {
        let product_unpack![range, k] = args;
        static_map(hash_map(any_with::<K>(k), base, range.into()),
            HashMap::into_iter)
    }
);

impl<A: Debug + Eq + Hash, B: Debug> functor::ArbitraryF2<A, B>
for HashMap<A, B> {
    type Parameters = SizeBounds;

    fn lift2_with<AS, BS>(fst: AS, snd: BS, args: Self::Parameters)
        -> BoxedStrategy<Self>
    where
        AS: Strategy + 'static,
        AS::Value: ValueTree<Value = A>,
        BS: Strategy + 'static,
        BS::Value: ValueTree<Value = B>
    {
        hash_map(fst, snd, args.into()).boxed()
    }
}

impl<A: Debug + Eq + Hash + 'static, B: 'static + Debug>
    functor::ArbitraryF2<A, B>
for hash_map::IntoIter<A, B> {
    type Parameters = SizeBounds;

    fn lift2_with<AS, BS>(fst: AS, snd: BS, args: Self::Parameters)
        -> BoxedStrategy<Self>
    where
        AS: Strategy + 'static,
        AS::Value: ValueTree<Value = A>,
        BS: Strategy + 'static,
        BS::Value: ValueTree<Value = B>
    {
        static_map(hash_map(fst, snd, args.into()), HashMap::into_iter).boxed()
    }
}

//==============================================================================
// BTreeMap:
//==============================================================================

impl<A: Arbitrary + Ord, B: Arbitrary> Arbitrary for BTreeMap<A, B>
where
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

lift1!([, K: Ord + Arbitrary + 'static] BTreeMap<K, A>,
    RangedParams1<K::Parameters>;
    base, args => {
        let product_unpack![range, k] = args;
        btree_map(any_with::<K>(k), base, range.into())
    }
);

impl<A: Debug + Ord, B: Debug> functor::ArbitraryF2<A, B> for BTreeMap<A, B> {
    type Parameters = SizeBounds;
    fn lift2_with<AS, BS>(fst: AS, snd: BS, args: Self::Parameters)
        -> BoxedStrategy<Self>
    where
        AS: Strategy + 'static,
        AS::Value: ValueTree<Value = A>,
        BS: Strategy + 'static,
        BS::Value: ValueTree<Value = B>
    {
        btree_map(fst, snd, args.into()).boxed()
    }
}

impl<A: Arbitrary + Ord, B: Arbitrary> Arbitrary for btree_map::IntoIter<A, B>
where
    StrategyFor<A>: 'static,
    StrategyFor<B>: 'static,
{
    valuetree!();
    type Parameters = <BTreeMap<A, B> as Arbitrary>::Parameters;
    type Strategy = SMapped<BTreeMap<A, B>, Self>;
    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        any_with_smap(args, BTreeMap::into_iter)        
    }
}

impl<A: Debug + Ord + 'static, B: 'static + Debug>
    functor::ArbitraryF2<A, B>
for btree_map::IntoIter<A, B> {
    type Parameters = SizeBounds;

    fn lift2_with<AS, BS>(fst: AS, snd: BS, args: Self::Parameters)
        -> BoxedStrategy<Self>
    where
        AS: Strategy + 'static,
        AS::Value: ValueTree<Value = A>,
        BS: Strategy + 'static,
        BS::Value: ValueTree<Value = B>
    {
        static_map(btree_map(fst, snd, args.into()), BTreeMap::into_iter).boxed()
    }
}

//==============================================================================
// Bound:
//==============================================================================

arbitrary!([A: Arbitrary] Bound<A>,
    TupleUnion<(
        W<SFnPtrMap<Arc<A::Strategy>, Self>>,
        W<SFnPtrMap<Arc<A::Strategy>, Self>>,
        W<LazyJustFn<Self>>
    )>,
    A::Parameters;
    args => {
        let base = Arc::new(any_with::<A>(args));
        prop_oneof![
            2 => static_map(base.clone(), Bound::Included),
            2 => static_map(base, Bound::Excluded),
            1 => LazyJust::new(|| Bound::Unbounded),
        ]
    }
);

lift1!(['static] Bound<A>; base => {
    let base = Rc::new(base);
    prop_oneof![
        2 => base.clone().prop_map(Bound::Included),
        2 => base.prop_map(Bound::Excluded),
        1 => LazyJustFn::new(|| Bound::Unbounded),
    ]
});

#[cfg(test)]
mod test {
    no_panic_test!(
        vec => Vec<u8>,
        box_slice => Box<[u8]>,
        rc_slice  => Rc<[u8]>,
        arc_slice  => Arc<[u8]>,
        vec_deque => VecDeque<u8>,
        linked_list => LinkedList<u8>,
        btree_set => BTreeSet<u8>,
        btree_map => BTreeMap<u8, u8>,
        hash_set => HashSet<u8>,
        hash_map => HashMap<u8, u8>,
        bound => Bound<u8>,
        binary_heap => BinaryHeap<u8>,
        into_iter_vec => vec::IntoIter<u8>,
        into_iter_vec_deque => vec_deque::IntoIter<u8>,
        into_iter_linked_list => linked_list::IntoIter<u8>,
        into_iter_binary_heap => binary_heap::IntoIter<u8>,
        into_iter_btree_set => btree_set::IntoIter<u8>,
        into_iter_btree_map => btree_map::IntoIter<u8, u8>,
        into_iter_hash_set => hash_set::IntoIter<u8>,
        into_iter_hash_map => hash_map::IntoIter<u8, u8>
    );
}