//==============================================================================
// Macros for quick implementing:
//==============================================================================

macro_rules! valuetree {
    () => {
        type ValueTree =
            <Self::Strategy as $crate::proptest::strategy::Strategy>::Value;
    };
}

macro_rules! arbitrary_for {
    ([$($bound : tt)*] $typ: ty, $strat: ty, $params: ty,
        $args: ident => $logic: expr) => {
        impl<'a, $($bound)*> $crate::Arbitrary<'a> for $typ {
            valuetree!();
            type Parameters = $params;
            type Strategy = $strat;
            fn arbitrary_with($args: Self::Parameters) -> Self::Strategy {
                $logic
            }
        }
    };
}

macro_rules! impl_arbitrary {
    ($self: ty, $st: ty, $logic: expr) => {
        arbitrary_for!([] $self, $st, (), _args => $logic);
    };
}

macro_rules! impl_just {
    ($self: ty, $logic: expr) => {
        impl_arbitrary!($self,
            $crate::proptest::strategy::Just<Self>,
            $crate::proptest::strategy::Just($logic));
    };
}

macro_rules! impls {
    ($($self: ident),*) => {
        $(impl_arbitrary!($self, $self::Any, $self::ANY);)*
    };
}

macro_rules! impl_wrap_gen {
    ([$($bound : tt)*] $wrap: ident) => {
        impl_wrap_gen!([$($bound)*] $wrap, $wrap::new);
    };
    ([$($bound : tt)*] $wrap: ident, $maker: expr) => {
        arbitrary_for!([A: $crate::Arbitrary<'a> + $($bound)*] $wrap<A>,
            $crate::SMapped<'a, A, Self>, A::Parameters,
            args => $crate::any_with_smap(args, $maker));
    };
}

macro_rules! impl_wrap_from {
    ([$($bound : tt)*] $wrap: ident) => {
        arbitrary_for!([A: $crate::Arbitrary<'a> + $($bound)*] $wrap<A>,
            $crate::FMapped<'a, A, Self>, A::Parameters,
            args => $crate::any_with_sinto::<A, $wrap<A>>(args));
    };
}

macro_rules! gen_strat {
    ($($self: ty, $fun: expr);+) => {
        $(
            impl_arbitrary!($self,
                $crate::GenStrategy<Self>, $crate::GenStrategy::new($fun));
        )+
    };
}