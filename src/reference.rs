
//==============================================================================
// Reference types:
//==============================================================================

use std::fmt::Debug;

use std::marker::PhantomData;
use proptest::test_runner::TestRunner;

pub trait ValueTree<'a> {
    type Value: Debug;
    fn current(&'a self) -> Self::Value;
    fn simplify(&mut self) -> bool;
    fn complicate(&mut self) -> bool;
}

pub trait Strategy<'a>: Debug {
    type Value: ValueTree<'a>;
    fn new_value(&self, runner: &mut TestRunner) -> Result<Self::Value, String>;
}


type VTValueFor<'a, VT> = <VT as ValueTree<'a>>::Value;
type ValueFor<'a, S> = VTValueFor<'a, <S as Strategy<'a>>::Value>;
type RefPH<'a, T> = PhantomData<&'a T>;

/*
#[derive(Debug)]
pub struct RefStrategy<'a, S: Strategy<'a> + 'a>
where
    ValueFor<'a, S>: 'a
{
    __marker: RefPH<'a, ValueFor<'a, S>>,
    strategy: S,
}

impl<'a, S: Strategy<'a>> Strategy<'a> for RefStrategy<'a, S> {
    type Value = RefValueTree<'a, S::Value>;

    fn new_value(&self, r: &mut TestRunner) -> Result<Self::Value, String> {
        self.strategy.new_value(r).map(RefValueTree::new)
    }
}
*/

pub struct RefValueTree<'a, VT: ValueTree<'a>>
where
    VTValueFor<'a, VT>: 'a
{
    vcurrent: Option<VTValueFor<'a, VT>>,
    inner_vt: VT,
}

impl<'a, VT: ValueTree<'a> + 'a> RefValueTree<'a, VT> {
    fn new(inner: VT) -> Self {
        Self {
            vcurrent: None,
            inner_vt: inner,
        }
    }
}

impl<'a, VT: ValueTree<'a>> ValueTree<'a> for RefValueTree<'a, VT> {
    type Value = &'a VT::Value;

    fn current(&'a self) -> Self::Value {
        self.vcurrent.as_ref().unwrap()//.as_ref()//.current()
    }

    fn simplify(&mut self) -> bool {
        unimplemented!()
    }

    fn complicate(&mut self) -> bool {
        unimplemented!()
    }
}