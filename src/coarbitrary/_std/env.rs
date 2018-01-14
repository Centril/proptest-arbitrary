use coarbitrary::*;

use std::env::*;

impl CoArbitrary for Args {
    fn coarbitrary(&self, var: Perturbable) {
        coarbitrary_iter(args(), var)
    }
}

impl CoArbitrary for ArgsOs {
    fn coarbitrary(&self, var: Perturbable) {
        coarbitrary_iter(args_os(), var)
    }
}

impl CoArbitrary for Vars {
    fn coarbitrary(&self, var: Perturbable) {
        coarbitrary_iter(vars(), var)
    }
}

impl CoArbitrary for VarsOs {
    fn coarbitrary(&self, var: Perturbable) {
        coarbitrary_iter(vars_os(), var)
    }
}

impl CoArbitrary for VarError {
    fn coarbitrary(&self, mut var: Perturbable) {
        match *self {
            VarError::NotPresent => var.variant(0),
            VarError::NotUnicode(ref ostr) => var.variant(1).nest(ostr),
        };
    }
}