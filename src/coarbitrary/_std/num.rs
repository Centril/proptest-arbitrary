use coarbitrary::*;

use std::num::*;

impl CoArbitrary for FpCategory {
    fn coarbitrary(&self, mut var: Perturbable) {
        match *self {
            FpCategory::Nan => var.variant(0),
            FpCategory::Infinite => var.variant(1),
            FpCategory::Zero => var.variant(2),
            FpCategory::Subnormal => var.variant(3),
            FpCategory::Normal => var.variant(4),
        };
    }
}

#[cfg(feature = "unstable")]
coarbitrary_unit!(TryFromIntError);

impl<T: CoArbitrary> CoArbitrary for Wrapping<T> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.0);
    }
}
