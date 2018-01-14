use coarbitrary::*;

use std::ops::*;

impl<A: CoArbitrary> CoArbitrary for Range<A> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.start).nest(&self.end);
    }
}

impl<A: CoArbitrary> CoArbitrary for RangeFrom<A> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.start);
    }
}

coarbitrary_unit!(RangeFull);

#[cfg(feature = "unstable")]
impl<A: CoArbitrary> CoArbitrary for RangeInclusive<A> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.start).nest(&self.end);
    }
}

impl<A: CoArbitrary> CoArbitrary for RangeTo<A> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.end);
    }
}

#[cfg(feature = "unstable")]
impl<A: CoArbitrary> CoArbitrary for RangeToInclusive<A> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.end);
    }
}

#[cfg(feature = "unstable")]
impl<Y: CoArbitrary, R: CoArbitrary> CoArbitrary for GeneratorState<Y, R> {
    fn coarbitrary(&self, mut var: Perturbable) {
        match *self {
            GeneratorState::Yielded(ref a) => var.variant(0).nest(a),
            GeneratorState::Complete(ref a) => var.variant(1).nest(a),
        };
    }
}