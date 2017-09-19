//==============================================================================
// Macros for quick implementing:
//==============================================================================

macro_rules! valuetree {
    () => {
        type ValueTree = <Self::Strategy as Strategy>::Value;
    };
}

macro_rules! impl_arbitrary {
    ($self: ty, $st: ty, $logic: expr) => {
        impl<'a> Arbitrary<'a> for $self {
            valuetree!();
            type Strategy = $st;
            fn arbitrary() -> Self::Strategy { $logic }
        }
    };
}

macro_rules! impls {
    ($($self: ident),*) => {
        $(impl_arbitrary!($self, $self::Any, $self::ANY);)*
    };
}

macro_rules! impl_unary {
    ($typ: ident, $strat: ident, $($bound : path),* => $logic: expr) => {
        impl<'a, A: Arbitrary<'a> $(+ $bound)*> Arbitrary<'a> for $typ<A> {
            valuetree!();
            type Strategy = $strat<A::Strategy>;
            fn arbitrary() -> Self::Strategy {
                $logic
            }
        }
    };
}

macro_rules! impl_binary {
    ($typ: ident, $strat: ident, $($bound : path),* => $logic: expr) => {
        impl<'a, A: Arbitrary<'a> $(+ $bound)* , B: Arbitrary<'a>> Arbitrary<'a>
        for $typ<A, B> {
            valuetree!();
            type Strategy = $strat<A::Strategy, B::Strategy>;
            fn arbitrary() -> Self::Strategy {
                $logic
            }
        }
    };
}