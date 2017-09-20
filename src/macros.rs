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