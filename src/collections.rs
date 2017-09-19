//==============================================================================
// Collections:
//==============================================================================

use super::*;

use std::hash::Hash;
use std::vec::Vec;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

use proptest::collection::{self, binary_heap, btree_set, hash_map, hash_set, linked_list, vec,
                           vec_deque, BTreeSetStrategy, BinaryHeapStrategy, HashMapStrategy,
                           BTreeMapStrategy, HashSetStrategy, LinkedListStrategy, VecDequeStrategy, VecStrategy};

impl_unary!(Vec, VecStrategy, => vec(arbitrary(), 0..100));
impl_unary!(VecDeque, VecDequeStrategy, => vec_deque(arbitrary(), 0..100));
impl_unary!(LinkedList, LinkedListStrategy, => linked_list(arbitrary(), 0..100));
impl_unary!(BTreeSet, BTreeSetStrategy, Ord => btree_set(arbitrary(), 0..100));
impl_unary!(BinaryHeap, BinaryHeapStrategy, Ord => {
    binary_heap(arbitrary(), 0..100)
});
impl_unary!(HashSet, HashSetStrategy, Hash, Eq => {
    hash_set(arbitrary(), 0..100)
});
impl_binary!(HashMap, HashMapStrategy, Hash, Eq => {
    hash_map(arbitrary(), arbitrary(), 0..100)
});

impl<'a, A, B> Arbitrary<'a> for BTreeMap<A, B>
where
    A: Arbitrary<'static> + Ord,
    B: Arbitrary<'static>,
    StrategyFor<A>: 'static,
    StrategyFor<B>: 'static,
{
    valuetree!();
    type Strategy = BTreeMapStrategy<A::Strategy, B::Strategy>;
    fn arbitrary() -> Self::Strategy {
        collection::btree_map(arbitrary(), arbitrary(), 0..100)
    }
}