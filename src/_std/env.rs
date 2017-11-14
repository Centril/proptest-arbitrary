use super::*;
use extras::GenStrategy;
use std::env;

impl_arbitrary!(env::Args, GenStrategy<Self>, GenStrategy::new(env::args));
impl_arbitrary!(env::ArgsOs, GenStrategy<Self>, GenStrategy::new(env::args_os));
impl_arbitrary!(env::Vars, GenStrategy<Self>, GenStrategy::new(env::vars));
impl_arbitrary!(env::VarsOs, GenStrategy<Self>, GenStrategy::new(env::vars_os));
impl_arbitrary!(env::JoinPathsError, GenStrategy<Self>, GenStrategy::new(jpe));

#[cfg(not(target_os = "windows"))]
fn jpe() -> env::JoinPathsError {
    env::join_paths(::std::iter::once(":")).unwrap_err()
}

#[cfg(target_os = "windows")]
fn jpe() -> env::JoinPathsError {
    env::join_paths(::std::iter::once(":")).unwrap_err()
}

// TODO: VarError once OsString is Arbitrary.

// TODO: SplitPaths when lifetimes in strategies are possible.