//! Arbitrary implementations for `std::hash`.

use super::*;
use std::heap::*;
use std::usize;

arbitrary!(CannotReallocInPlace; CannotReallocInPlace);
arbitrary!(Heap; Heap);

// Not Debug :/
//generator!(System, || System);

arbitrary!(Layout, BoxedStrategy<Layout>;
    (0u8..32u8).prop_flat_map(|align_power| {
        // align must be a power of two and <= (1 << 31):
        let align = 1 << align_power;
        // Compute the highest multiple of align <= usize::MAX:
        // By definition an integer since 2^X / 2^Y = 2^(X - Y)
        // X, Y are integers, X >= Y, so X - Y is a positive integer,
        // so 2^(X - Y) is too.
        let max_size = usize::MAX / align;
        // Should perhaps be ..=max_size but we can't express that now.
        (..max_size).prop_map(move |size|
            Layout::from_size_align(size, align).unwrap())
    }).boxed()
);

arbitrary!(AllocErr, TupleUnion<(W<SMapped<Layout, Self>>, W<Just<Self>>)>;
    prop_oneof![
        static_map(any::<Layout>(), |request| AllocErr::Exhausted { request }),
        Just(AllocErr::Unsupported {
            // We could randomly generate a string and then leak it, but let's
            // not do that since might run out of memory in testing or otherwise
            // make the TestRunner really slow.
            details: "<Unsupported>"
        })
    ]
);

#[cfg(test)]
mod test {
    no_panic_test!(
        layout => Layout,
        alloc_err => AllocErr
    );
}