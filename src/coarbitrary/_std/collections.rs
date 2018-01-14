//! CoArbitrary for ::std::collections.

use coarbitrary::*;

use std::vec;
use std::collections::*;
use std::hash::{Hash, BuildHasher};

impl<A: CoArbitrary> CoArbitrary for Bound<A> {
    fn coarbitrary(&self, mut var: Perturbable) {
        match *self {
            Bound::Included(ref x) => var.variant(0).nest(x),
            Bound::Excluded(ref x) => var.variant(1).nest(x),
            Bound::Unbounded => var.variant(2),
        };
    }
}

//==============================================================================
// Vec
//==============================================================================

impl<A: CoArbitrary> CoArbitrary for vec::Vec<A> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.capacity()).nest(&**self);
    }
}

impl<A: CoArbitrary> CoArbitrary for vec::IntoIter<A> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(self.as_slice());
    }
}

//==============================================================================
// VecDeque
//==============================================================================

delegate_iter!([A: CoArbitrary] vec_deque::VecDeque<A>, iter);
delegate_iter!([A: CoArbitrary + Clone] vec_deque::IntoIter<A>);
delegate_iter!(['a, A: CoArbitrary] vec_deque::Iter<'a, A>);

//==============================================================================
// LinkedList
//==============================================================================

delegate_iter!([A: CoArbitrary] linked_list::LinkedList<A>, iter);
delegate_iter!([A: CoArbitrary + Clone] linked_list::IntoIter<A>);
delegate_iter!(['a, A: CoArbitrary] linked_list::Iter<'a, A>);

//==============================================================================
// BinaryHeap
//==============================================================================

delegate_iter!([A: CoArbitrary + Ord] binary_heap::BinaryHeap<A>, iter);
delegate_iter!([A: CoArbitrary + Clone] binary_heap::IntoIter<A>);
delegate_iter!(['a, A: CoArbitrary] binary_heap::Iter<'a, A>);

impl<'a, A: CoArbitrary + Ord> CoArbitrary for binary_heap::PeekMut<'a, A> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&**self);
    }
}

//==============================================================================
// BTreeSet
//==============================================================================

delegate_iter!([A: CoArbitrary] btree_set::BTreeSet<A>, iter);
delegate_iter!(['a, A: CoArbitrary + Ord] btree_set::Difference<'a, A>);
delegate_iter!(['a, A: CoArbitrary + Ord] btree_set::Intersection<'a, A>);
delegate_iter!(['a, A: CoArbitrary + Ord] btree_set::SymmetricDifference<'a, A>);
delegate_iter!(['a, A: CoArbitrary + Ord] btree_set::Union<'a, A>);
delegate_iter!(['a, A: CoArbitrary] btree_set::Iter<'a, A>);
delegate_iter!(['a, A: CoArbitrary] btree_set::Range<'a, A>);

//==============================================================================
// BTreeMap
//==============================================================================

delegate_iter!([K: CoArbitrary + Ord, V: CoArbitrary] BTreeMap<K, V>, iter);
delegate_iter!(['a, K: CoArbitrary, V: CoArbitrary] btree_map::Iter<'a, K, V>);
delegate_iter!(['a, K: CoArbitrary, V] btree_map::Keys<'a, K, V>);
delegate_iter!(['a, K: CoArbitrary, V: CoArbitrary] btree_map::Range<'a, K, V>);
delegate_iter!(['a, K, V: CoArbitrary] btree_map::Values<'a, K, V>);

impl<'a, K: CoArbitrary + Ord, V: CoArbitrary> CoArbitrary
for btree_map::Entry<'a, K, V> {
    fn coarbitrary(&self, mut var: Perturbable) {
        match *self {
            btree_map::Entry::Vacant(ref x) => var.variant(0).nest(x),
            btree_map::Entry::Occupied(ref x) => var.variant(1).nest(x),
        };
    }
}

impl<'a, K: CoArbitrary + Ord, V> CoArbitrary
for btree_map::VacantEntry<'a, K, V> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(self.key());
    }
}

impl<'a, K: CoArbitrary + Ord, V: CoArbitrary> CoArbitrary
for btree_map::OccupiedEntry<'a, K, V> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(self.key()).nest(self.get());
    }
}

//==============================================================================
// HashSet
//==============================================================================

delegate_iter!([A: CoArbitrary + Eq + Hash, S: BuildHasher]
               hash_set::HashSet<A, S>, iter);
delegate_iter!(['a, A: CoArbitrary + Eq + Hash, S: BuildHasher]
               hash_set::Difference<'a, A, S>);
delegate_iter!(['a, A: CoArbitrary + Eq + Hash, S: BuildHasher]
               hash_set::Intersection<'a, A, S>);
delegate_iter!(['a, A: CoArbitrary + Eq + Hash, S: BuildHasher]
               hash_set::SymmetricDifference<'a, A, S>);
delegate_iter!(['a, A: CoArbitrary + Eq + Hash, S: BuildHasher]
               hash_set::Union<'a, A, S>);
delegate_iter!(['a, A: CoArbitrary] hash_set::Iter<'a, A>);

//==============================================================================
// HashMap
//==============================================================================

delegate_iter!([K: CoArbitrary + Eq + Hash, V: CoArbitrary, S: BuildHasher]
               HashMap<K, V, S>, iter);
delegate_iter!(['a, K: CoArbitrary, V: CoArbitrary] hash_map::Iter<'a, K, V>);
delegate_iter!(['a, K: CoArbitrary, V] hash_map::Keys<'a, K, V>);
delegate_iter!(['a, K, V: CoArbitrary] hash_map::Values<'a, K, V>);

impl<'a, K: CoArbitrary + Eq + Hash, V: CoArbitrary> CoArbitrary
for hash_map::Entry<'a, K, V> {
    fn coarbitrary(&self, mut var: Perturbable) {
        match *self {
            hash_map::Entry::Vacant(ref x) => var.variant(0).nest(x),
            hash_map::Entry::Occupied(ref x) => var.variant(1).nest(x),
        };
    }
}

impl<'a, K: CoArbitrary + Eq + Hash, V> CoArbitrary
for hash_map::VacantEntry<'a, K, V> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(self.key());
    }
}

impl<'a, K: CoArbitrary + Eq + Hash, V: CoArbitrary> CoArbitrary
for hash_map::OccupiedEntry<'a, K, V> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(self.key()).nest(self.get());
    }
}