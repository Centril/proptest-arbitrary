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

macro_rules! params_unary {
    ($type: ident) => {
        /// Parameters for configuring the generation of `StrategyFor<...<A>>`.
        #[derive(Clone, PartialEq, Debug)]
        pub struct $type<A, B> {
            aux: B,
            a_params: A,
        }

        impl<A: Default, B: Default> Default for $type<A, B> {
            fn default() -> Self {
                (def(),).into()
            }
        }

        // No info => Default:
        impl<A: Default, B: Default> From<()> for $type<A, B> {
            fn from(_: ()) -> Self {
                Self::default()
            }
        }

        impl<A: Default, X: From<XF>, XF> From<XF> for $type<A, X> {
            fn from(x: XF) -> Self {
                (x, def()).into()
            }
        }

        impl<AF, A: From<AF>, X: Default> From<(AF,)> for $type<A, X> {
            fn from(x: (AF,)) -> Self {
                (def::<X>(), x.0).into()
            }
        }

        impl<AF, A: From<AF>, XF, X: From<XF>> From<(XF, AF)> for $type<A, X> {
            fn from(x: (XF, AF)) -> Self {
                Self {
                    aux: x.0.into(),
                    a_params: x.1.into(),
                }
            }
        }
    };
}
