//! Arbitrary implementations for `std::thread`.

use super::*;
use std::thread::*;

arbitrary!(Builder, SMapped<'a, (Option<usize>, Option<String>), Self>; {
    let prob = prob(0.7);
    let args = product_pack![
        product_pack![prob, default()],
        product_pack![prob, default()]
    ];
    any_with_smap(args, |(os, on)| {
        let mut b = Builder::new();
        b = if let Some(size) = os { b.stack_size(size) } else { b };
        if let Some(name) = on { b.name(name) } else { b }
    })
});

/*
 * The usefulness of this impl is debatable - as are its semantics.
 * Perhaps a CoArbitrary-based solution is preferable.

arbitrary!([A: 'static + Send + Arbitrary<'a>] JoinHandle<A>,
    SMapped<'a, (A, Option<()>, u8), Self>, A::Parameters;
    args => {
        let prob  = prob(0.1);
        let args2 = product_pack![
            args, 
            product_pack![prob, default()],
            default()
        ];
        any_with_smap(args2, |(val, panic, sleep)| thread::spawn(move || {
            // Sleep a random amount:
            use std::time::Duration;
            thread::sleep(Duration::from_millis(sleep as u64));

            // Randomly panic:
            if panic.is_some() {
                panic!("Arbitrary for JoinHandle randomly paniced!");
            }

            // Move value into thread and then just return it:
            val
        }))
    }
);
*/

#[cfg(feature = "unstable")]
arbitrary!(LocalKeyState,
    TupleUnion<(W<Just<Self>>, W<Just<Self>>, W<Just<Self>>)>;
    prop_oneof![
        Just(LocalKeyState::Uninitialized),
        Just(LocalKeyState::Valid),
        Just(LocalKeyState::Destroyed)
    ]
);

#[cfg(test)]
mod test {
    no_panic_test!(
        builder => Builder
    );

    #[cfg(feature = "unstable")]
    no_panic_test!(
        local_key_state => LocalKeyState
    );

    /*
    use super::*;
    proptest! {
        #[test]
        fn join_handle_works(ref jh in any::<JoinHandle<u8>>()) {
            use std::panic::catch_unwind;
            catch_unwind(|| {
                jh.join();
                ()
            })
        }
    }
    */
}