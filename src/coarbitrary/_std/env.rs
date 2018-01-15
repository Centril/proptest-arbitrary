use coarbitrary::*;

use std::env::*;

coarbitrary!(Args; self, var => coarbitrary_iter(args(), var));
coarbitrary!(ArgsOs; self, var => coarbitrary_iter(args_os(), var));
coarbitrary!(Vars; self, var => coarbitrary_iter(vars(), var));
coarbitrary!(VarsOs; self, var => coarbitrary_iter(vars_os(), var));
coarbitrary!(VarError; self, var => match *self {
    VarError::NotPresent => var.variant(0),
    VarError::NotUnicode(ref ostr) => var.variant(1).nest(ostr),
});