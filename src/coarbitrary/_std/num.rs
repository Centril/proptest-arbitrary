use coarbitrary::*;

use std::num::*;

coarbitrary!(FpCategory; self, var => match *self {
    FpCategory::Nan => var.variant(0),
    FpCategory::Infinite => var.variant(1),
    FpCategory::Zero => var.variant(2),
    FpCategory::Subnormal => var.variant(3),
    FpCategory::Normal => var.variant(4),
});

#[cfg(feature = "unstable")]
coarbitrary_unit!(TryFromIntError);

coarbitrary!([T: CoArbitrary] Wrapping<T>; self, var => var.nest(&self.0));