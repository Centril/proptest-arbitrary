//! Arbitrary implementations for `std::option`.

use super::*;
use std::option as opt;
use proptest::option::{self, OptionStrategy};

arbitrary!([A: Arbitrary] opt::Option<A>, OptionStrategy<A::Strategy>,
    product_type![Probability, A::Parameters];
    args => {
        let product_unpack![prob, a] = args;
        option::weighted(prob.into(), any_with::<A>(a))
    }
);

lift1!([] opt::Option<A>, Probability;
    base, prob => option::weighted(prob.into(), base)
);

arbitrary!([A: Arbitrary] opt::IntoIter<A>, SMapped<opt::Option<A>, Self>,
    <opt::Option<A> as Arbitrary>::Parameters;
    args => any_with_smap(args, Option::into_iter));

lift1!(['static] opt::IntoIter<A>, Probability;
    base, prob => option::weighted(prob.into(), base)
                    .prop_map(Option::into_iter)
);

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