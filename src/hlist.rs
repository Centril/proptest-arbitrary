pub(crate) fn def<D: Default>() -> D {
    D::default()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct HCons<H, T>(H, T);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct HNil;

pub trait Nat {}
impl Nat for Z {}
impl Nat for S {}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Z {}

use std::marker::PhantomData;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct S<T = Z>(PhantomData<T>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Region;

impl Default for HNil {
    fn default() -> Self {
        HNil
    }
}

impl<T: Default, Tail: Default> Default for HCons<T, Tail> {
    fn default() -> Self {
        HCons(def(), def())
    }
}




pub trait Reach<T> {
    fn reach() -> T;
}

impl<T> Reach<T> for T {
    fn reach() -> T {
        panic!()
    }
}

pub trait Create<T, I> {
    type Index: Reach<I>;

    fn make(x: T) -> Self;
}

impl<T, Tail: Default> Create<T, Z> for HCons<T, Tail> {
    type Index = Z;

    fn make(x: T) -> Self {
        HCons(x, def())
    }
}

impl<Head, Tail, FromTail, TailIndex> Create<FromTail, S<TailIndex>> for HCons<Head, Tail>
where
    Head: Default,
    Tail: Create<FromTail, TailIndex>,
{
    type Index = S<TailIndex>;

    fn make(x: FromTail) -> Self {
        HCons(def(), Tail::make(x))
    }
}

impl<T: Default, Tail> Create<Tail, Region> for HCons<T, Tail> {
    type Index = Region;

    fn make(x: Tail) -> Self {
        HCons(def(), x)
    }
}

fn make<I, T, HL: Create<T, I>>(x: T) -> HL {
    HL::make(x)
}

fn main() {
    type H1 = HCons<(), HNil>;
    type H2 = HCons<f64, H1>;
    type H3 = HCons<usize, H2>;
    type H4 = HCons<(), H3>;
    //let x: H3 = make(HCons(-1, HNil));
    let x: H4 = make(1); //(1);

    //let x: H3 = make(HCons(1337.0, HNil));//HCons((), HNil)));
    println!("{:?}", x);
}
