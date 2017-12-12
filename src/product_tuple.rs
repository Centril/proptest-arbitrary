//! Defines macros for product type creation, extraction, and the type signature
//! itself. This version uses tuples. This mechanism is used to be very
//! loosely coupled with `frunk_core` so that only `lib.rs` has to be changed
//! in the event that Rust gets tuple-varadic generics.

macro_rules! product_type {
    ($factor: ty) => {
        ($factor,)
    };
    ($($factor: ty),*) => {
        ( $( $factor, )* )
    };
    ($($factor: ty),*,) => {
        ( $( $factor, )* )
    };
}

macro_rules! product_pack {
    ($factor: expr) => {
        ($factor,)
    };
    ($($factor: expr),*) => {
        ( $( $factor ),* )
    };
    ($($factor: expr),*,) => {
        ( $( $factor ),* )
    };
}

macro_rules! product_unpack {
    ($factor: pat) => {
        ($factor,)
    };
    ($($factor: pat),*) => {
        ( $( $factor ),* )
    };
    ($($factor: pat),*,) => {
        ( $( $factor ),* )
    };
}