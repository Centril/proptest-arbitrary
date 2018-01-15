use coarbitrary::*;

use std::ops::*;

coarbitrary!([A: CoArbitrary] Range<A>;
    self, var => var.nest(&self.start).nest(&self.end));
coarbitrary!([A: CoArbitrary] RangeFrom<A>;
    self, var => var.nest(&self.start));
coarbitrary_unit!(RangeFull);
#[cfg(feature = "unstable")]
coarbitrary!([A: CoArbitrary] RangeInclusive<A>;
    self, var => var.nest(&self.start).nest(&self.end));
coarbitrary!([A: CoArbitrary] RangeTo<A>;
    self, var => var.nest(&self.end));
#[cfg(feature = "unstable")]
coarbitrary!([A: CoArbitrary] RangeToInclusive<A>;
    self, var => var.nest(&self.end));
#[cfg(feature = "unstable")]
coarbitrary!([Y: CoArbitrary, R: CoArbitrary] GeneratorState<Y, R>;
    self, var => match *self {
        GeneratorState::Yielded(ref a) => var.variant(0).nest(a),
        GeneratorState::Complete(ref a) => var.variant(1).nest(a),
    }
);