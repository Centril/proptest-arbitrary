#[cfg(feature = "unstable")]
use coarbitrary::*;

#[cfg(feature = "unstable")]
impl CoArbitrary for ::std::convert::Infallible {
    fn coarbitrary(&self, _: Perturbable) { match self {} }
}
