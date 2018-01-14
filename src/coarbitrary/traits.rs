//! Defines the `CoArbitrary` trait and the
//! `Perturbable` helper type over `XorShiftRng`.

use proptest::prelude::XorShiftRng;

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

//==============================================================================
// Perturbing helpers:
//==============================================================================.

/// Perturbs the RNG with an integer seed.
fn variant(rng: &mut XorShiftRng, seed: u32) {
    // See: https://github.com/rust-lang-nursery/rand/issues/231
    use rand::{Rand, SeedableRng};
    let mut seed2 = <[u32; 4] as Rand>::rand(rng);
    seed2[0] = seed2[0].wrapping_add(seed);
    rng.reseed(seed2);
}

/// Helper for convenient defining of `CoArbitrary`.
pub struct Perturbable<'rng> {
    rng: &'rng mut XorShiftRng
}

impl<'rng> Perturbable<'rng> {
    /// Constructs a new helper given the RNG to perturb
    /// in calls to `.coarbitrary(<the constructed object>)`.
    pub fn new(rng: &'rng mut XorShiftRng) -> Self {
        Self { rng }
    }

    /// Returns the underlying RNG.
    pub fn rng(self) -> &'rng mut XorShiftRng {
        self.rng
    }

    /// Perturbs the underlying RNG with an integer seed.
    pub fn variant(&mut self, seed: u32) -> &mut Self {
        variant(self.rng, seed);
        self
    }

    /// Perturbs the underlying RNG using the `CoArbitrary`
    /// implementation of the given object.
    pub fn nest<C: CoArbitrary + ?Sized>(&mut self, nested: &C) -> &mut Self {
        nested.coarbitrary(Self::new(self.rng));
        self
    }
}

/// Helper to define coarbitrary based on the elements in a sequence.
pub (crate) fn coarbitrary_iter<A, I>(iter: I, mut var: Perturbable)
where
    A: CoArbitrary,
    I: Iterator<Item = A>,
{
    for x in iter { var.variant(1).nest(&x); }
    var.variant(0);
}

/// Helper to define coarbitrary based on the hash of an object.
/// This is usually good when this is the only possible implementation.
pub (crate) fn coarbitrary_hash<H: Hash>(hashable: H, mut var: Perturbable) {
    let mut hasher = DefaultHasher::default();
    hashable.hash(&mut hasher);
    let hash = hasher.finish();
    var.nest(&hash);
}

//==============================================================================
// CoArbitrary:
//==============================================================================.

/// `CoArbitrary` defines a method for perturbing an RNG
/// based on the value of `&self`.
///
/// Note that the implementation must always be pure,
/// in other words: Given the same input `&self`, and
/// RNG the value of the RNG must be the same after
/// calling `.coarbitrary(..)`.
///
/// Other side effects that modify the world in important
/// ways should also be avoided.
pub trait CoArbitrary {
    /// Perturbs the given underlying RNG according to
    /// the structure of the given `&self` value.
    fn coarbitrary(&self, var: Perturbable);
}
