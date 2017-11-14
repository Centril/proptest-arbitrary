//==============================================================================
// Macros for quick implementing:
//==============================================================================

macro_rules! valuetree {
    () => {
        type ValueTree = <Self::Strategy as Strategy>::Value;
    };
}

macro_rules! params {
    () => {
        type Parameters = ();
    }
}

macro_rules! impl_arbitrary {
    ($self: ty, $st: ty, $logic: expr) => {
        impl<'a> Arbitrary<'a> for $self {
            valuetree!();
            params!();
            type Strategy = $st;
            fn arbitrary_with(_: Self::Parameters) -> Self::Strategy { $logic }
        }
    };
}

macro_rules! impls {
    ($($self: ident),*) => {
        $(impl_arbitrary!($self, $self::Any, $self::ANY);)*
    };
}

macro_rules! impl_wrap_gen {
    ($wrap: ident $(,$bound : path)*) => {
        impl<'a, A: $crate::Arbitrary<'a> $(+ $bound)*> $crate::Arbitrary<'a>
        for $wrap<A> {
            valuetree!();
            type Parameters = A::Parameters;
            type Strategy = $crate::from_mapper::Mapped<'a, A, Self>;
            fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
                $crate::any_with::<A, _>(args).prop_map($wrap::new)
            }
        }
    };
}

macro_rules! arbitrary_for {
    ($typ: ty [$($bound : tt)*] [$strat: ty] [$params: ty],
        $args: ident => $logic: block) => {
        impl<'a, $($bound)*> Arbitrary<'a> for $typ {
            valuetree!();
            type Parameters = $params;
            type Strategy = $strat;
            fn arbitrary_with($args: Self::Parameters) -> Self::Strategy {
                $logic
            }
        }
    };
}