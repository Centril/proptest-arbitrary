//==============================================================================
// Macros for quick implementing:
//==============================================================================

macro_rules! default {
    ($type: ty, $val: expr) => {
        impl Default for $type {
            fn default() -> Self { $val.into() }
        }
    };
}

macro_rules! valuetree {
    () => {
        type ValueTree =
            <Self::Strategy as $crate::proptest::strategy::Strategy>::Value;
    };
}

macro_rules! arbitrary {
    ([$($bounds : tt)*] $typ: ty, $strat: ty, $params: ty;
        $args: ident => $logic: expr) => {
        impl<$($bounds)*> $crate::Arbitrary for $typ {
            valuetree!();
            type Parameters = $params;
            type Strategy = $strat;
            fn arbitrary_with($args: Self::Parameters) -> Self::Strategy {
                $logic
            }
        }
    };
    ([$($bounds : tt)*] $typ: ty, $strat: ty; $logic: expr) => {
        arbitrary!([$($bounds)*] $typ, $strat, (); _args => $logic);
    };
    ([$($bounds : tt)*] $strat: ty; $logic: expr) => {
        arbitrary!([$($bounds)*] $strat,
            $crate::proptest::strategy::Just<Self>, ();
            _args => $crate::proptest::strategy::Just($logic)
        );
    };
    ($typ: ty, $strat: ty, $params: ty; $args: ident => $logic: expr) => {
        arbitrary!([] $typ, $strat, $params; $args => $logic);
    };
    ($typ: ty, $strat: ty; $logic: expr) => {
        arbitrary!([] $typ, $strat; $logic);
    };
    ($strat: ty; $logic: expr) => {
        arbitrary!([] $strat; $logic);
    };
    ($($typ: ident),*) => {
        $(arbitrary!($typ, $typ::Any; $typ::ANY);)*
    };
}

macro_rules! wrap_ctor {
    ($wrap: ident) => {
        wrap_ctor!([] $wrap);
    };
    ($wrap: ident, $maker: expr) => {
        wrap_ctor!([] $wrap, $maker);
    };
    ([$($bound : tt)*] $wrap: ident) => {
        wrap_ctor!([$($bound)*] $wrap, $wrap::new);
    };
    ([$($bound : tt)*] $wrap: ident, $maker: expr) => {
        arbitrary!([A: $crate::Arbitrary + $($bound)*] $wrap<A>,
            $crate::SMapped<A, Self>, A::Parameters;
            args => $crate::any_with_smap(args, $maker));

        lift1!([$($bound)*] $wrap<A>; $maker);
    };
}

macro_rules! wrap_from {
    ($wrap: ident) => {
        wrap_from!([] $wrap);
    };
    ([$($bound : tt)*] $wrap: ident) => {
        arbitrary!([A: $crate::Arbitrary + $($bound)*] $wrap<A>,
            $crate::FMapped<A, Self>, A::Parameters;
            args => $crate::any_with_sinto::<A, $wrap<A>>(args));

        lift1!([$($bound)*] $wrap<A>);
    };
}

macro_rules! lazy_just {
    ($($self: ty, $fun: expr);+) => {
        $(
            arbitrary!($self, $crate::LazyJustFn<Self>;
                $crate::LazyJustFn::new($fun));
        )+
    };
}

//==============================================================================
// Macros for testing:
//==============================================================================

/// We are mostly interested in ensuring that generating input from our
/// strategies is able to construct a value, therefore ensuring that
/// no panic occurs is mostly sufficient. Shrinking for strategies that
/// use special shrinking methods can be handled separately.
#[cfg(test)]
macro_rules! no_panic_test {
    ($($module: ident => $self: ty),+) => {
        $(
            mod $module {
                #[allow(unused_imports)]
                use super::super::*;
                proptest! {
                    #[test]
                    fn no_panic(_ in $crate::any::<$self>()) {}
                }
            }
        )+
    };
}