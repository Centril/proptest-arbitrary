//! Defines macros for product type creation, extraction, and the type signature
//! itself. This version uses `frunk_core`. This mechanism is used to be very
//! loosely coupled with `frunk_core` so that only `lib.rs` has to be changed
//! in the event that Rust gets tuple-varadic generics.

macro_rules! product_type {
    ($factor: ty) => {
        Hlist![$factor]
    };
    ($($factor: ty),*) => {
        Hlist![$( $factor, )*]
    };
    ($($factor: ty),*,) => {
        Hlist![$( $factor, )*]
    };
}

macro_rules! product_pack {
    ($factor: expr) => {
        hlist![$factor]
    };
    ($($factor: expr),*) => {
        hlist![$( $factor ),*]
    };
    ($($factor: expr),*,) => {
        hlist![$( $factor ),*]
    };
}

macro_rules! product_unpack {
    ($factor: pat) => {
        hlist_pat![$factor]
    };
    ($($factor: pat),*) => {
        hlist_pat![$( $factor ),*]
    };
    ($($factor: pat),*,) => {
        hlist_pat![$( $factor ),*]
    };
}