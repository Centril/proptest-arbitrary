//! Defines CoArbitrary for primitive types, mutable and shared rferences,
//! arrays and tuples.

use coarbitrary::traits::*;

//==============================================================================
// Never type:
//==============================================================================.

#[cfg(feature = "unstable")]
impl CoArbitrary for ! {
    fn coarbitrary(&self, _: Perturbable) { match self {} }
}

//==============================================================================
// Primitive types:
//==============================================================================.

coarbitrary!(bool; self, var => var.variant(if *self { 1 } else { 0 }));

macro_rules! coarbitrary_unsized {
    ($($typ: ty),*) => {
        $(
            coarbitrary!($typ; self, var => var.variant(*self as u32));
        )*
    };
}

coarbitrary_unsized!(u8, u16, u32, i8, i16, i32);

coarbitrary!(u64; self, var =>
    var.variant((*self >> (0 * 32)) as u32)
       .variant((*self >> (1 * 32)) as u32));

coarbitrary!(i64; self, var => var.nest(&(*self as u64)));

impl CoArbitrary for usize {
    fn coarbitrary(&self, mut var: Perturbable) {
        #[cfg(target_pointer_width = "64")]
        var.nest(&(*self as u64));
        #[cfg(not(target_pointer_width = "32"))]
        var.nest(&(*self as u32));
    }
}

impl CoArbitrary for isize {
    fn coarbitrary(&self, mut var: Perturbable) {
        #[cfg(target_pointer_width = "64")]
        var.nest(&(*self as i64));
        #[cfg(not(target_pointer_width = "32"))]
        var.nest(&(*self as i32));
    }
}

/*
TODO: deal with this..

#[cfg(feature = "unstable")]
impl CoArbitrary for u128 {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.variant((*self >> (0 * 32)) as u32)
           .variant((*self >> (1 * 32)) as u32)
           .variant((*self >> (2 * 32)) as u32)
           .variant((*self >> (3 * 32)) as u32);
    }
}

#[cfg(feature = "unstable")]
impl CoArbitrary for u128 {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&(*self as i128));
    }
}
*/

coarbitrary!(char; self, var => var.nest(&u32::from(*self)));

//==============================================================================
// &str:
//==============================================================================.

coarbitrary!(['a] &'a str; self, var => var.nest(self.as_bytes()));

//==============================================================================
// Reference types:
//==============================================================================.

coarbitrary!(['a, A: CoArbitrary + ?Sized] &'a A;
    self, var => var.nest(*self));
coarbitrary!(['a, A: CoArbitrary + ?Sized] &'a mut A;
    self, var => var.nest(*self));

//==============================================================================
// Arrays:
//==============================================================================.

macro_rules! array {
    ($size: expr; $($idx: expr)*) => {
        coarbitrary!([A: CoArbitrary] [A; $size];
            self, _var => { $( _var.nest(&self[$idx]); )* });
    };
}

array!(0;);
array!(1;  0);
array!(2;  0 1);
array!(3;  0 1 2);
array!(4;  0 1 2 3);
array!(5;  0 1 2 3 4);
array!(6;  0 1 2 3 4 5);
array!(7;  0 1 2 3 4 5 6);
array!(8;  0 1 2 3 4 5 6 7);
array!(9;  0 1 2 3 4 5 6 7 8);
array!(10; 0 1 2 3 4 5 6 7 8 9);
array!(11; 0 1 2 3 4 5 6 7 8 9 10);
array!(12; 0 1 2 3 4 5 6 7 8 9 10 11);
array!(13; 0 1 2 3 4 5 6 7 8 9 10 11 12);
array!(14; 0 1 2 3 4 5 6 7 8 9 10 11 12 13);
array!(15; 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14);
array!(16; 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15);
array!(17; 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16);
array!(18; 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17);
array!(19; 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18);
array!(20; 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19);
array!(21; 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20);
array!(22; 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21);
array!(23; 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22);
array!(24; 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23);
array!(25; 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24);
array!(26; 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25);
array!(27; 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25
           26);
array!(28; 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25
           26 27);
array!(29; 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25
           26 27 28);
array!(30; 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25
           26 27 28 29);
array!(31; 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25
           26 27 28 29 30);
array!(32; 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25
           26 27 28 29 30 31);

//==============================================================================
// DST:
//==============================================================================.

delegate_iter!([A: CoArbitrary] [A], iter);

//==============================================================================
// Tuples:
//==============================================================================.

macro_rules! tuple_coarbitrary {
    ( $($ty: ident)* ) => {
        impl<$($ty : CoArbitrary),*> CoArbitrary for ($($ty,)*) {
            #[allow(unused_mut)]
            #[allow(non_snake_case)]
            fn coarbitrary(&self, mut _var: Perturbable) {
                let &($(ref $ty,)*) = self;
                $(_var.nest($ty);)*
            }
        }
    };
}

tuple_coarbitrary!();
tuple_coarbitrary!(T0);
tuple_coarbitrary!(T0 T1);
tuple_coarbitrary!(T0 T1 T2);
tuple_coarbitrary!(T0 T1 T2 T3);
tuple_coarbitrary!(T0 T1 T2 T3 T4);
tuple_coarbitrary!(T0 T1 T2 T3 T4 T5);
tuple_coarbitrary!(T0 T1 T2 T3 T4 T5 T6);
tuple_coarbitrary!(T0 T1 T2 T3 T4 T5 T6 T7);
tuple_coarbitrary!(T0 T1 T2 T3 T4 T5 T6 T7 T8);
tuple_coarbitrary!(T0 T1 T2 T3 T4 T5 T6 T7 T8 T9);

#[cfg(test)]
mod test {
    coarbitrary_pure!(
        bool,
        u8, u16, u32, u64, usize,
        i8, i16, i32, i64, isize,
        char
    );

    #[cfg(feature = "unstable")]
    coarbitrary_pure!(u128, i128);

    coarbitrary_pure!(
        tuple_0 => (),
        tuple_1 => (u8,),
        tuple_3 => (u8, bool, u64),
        array_3 => [u8; 3],
        array_32 => [u8; 32]
    );
}