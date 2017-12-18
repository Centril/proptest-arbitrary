//! Arbitrary implementations for `std::option`.

use super::*;
use std::option as opt;
use proptest::option::{self, OptionStrategy};

arbitrary!(
    [A: Arbitrary<'a>] opt::Option<A>,
    OptionStrategy<A::Strategy>,
    product_type![Probability, A::Parameters];
    args => {
        let product_unpack![prob, a] = args;
        option::weighted(prob.into(), any_with::<A>(a))
    }
);

arbitrary!([A: Arbitrary<'a>] opt::IntoIter<A>,
    SMapped<'a, opt::Option<A>, Self>,
    <opt::Option<A> as Arbitrary<'a>>::Parameters;
    args => any_with_smap(args, Option::into_iter));

#[cfg(feature = "unstable")]
arbitrary!(opt::NoneError; opt::NoneError);

#[cfg(test)]
mod test {
    no_panic_test!(
        option      => Option<u8>,
        option_iter => opt::IntoIter<u8>
    );

    #[cfg(feature = "unstable")]
    no_panic_test!(
        none_error => opt::NoneError
    );
}