//! Arbitrary implementations for `std::io`.

use super::*;
use std::io::*;
use std::io::ErrorKind::*;

// TODO: IntoInnerError
// Consider: std::io::Initializer

macro_rules! buffer {
    ($type: ident, $bound: path) => {
        arbitrary!(
            [A: Arbitrary + $bound] $type<A>,
            SMapped<(A, Option<u16>), Self>, A::Parameters;
            args => any_with_smap(product_pack![args, default()], |(inner, cap)|
                if let Some(cap) = cap {
                    $type::with_capacity(cap as usize, inner)
                } else {
                    $type::new(inner)
                }
            )
        );

        lift1!([$bound] $type<A>; base =>
            (base, any::<Option<u16>>()).prop_map(|(inner, cap)| {
                if let Some(cap) = cap {
                    $type::with_capacity(cap as usize, inner)
                } else {
                    $type::new(inner)
                }
            })
        );
    };
}

buffer!(BufReader,  Read);
buffer!(BufWriter,  Write);
buffer!(LineWriter, Write);

arbitrary!(
    [A: Read + Arbitrary, B: Read + Arbitrary] Chain<A, B>,
    SMapped<(A, B), Self>, product_type![A::Parameters, B::Parameters];
    args => any_with_smap(args, |(a, b)| a.chain(b))
);

wrap_ctor!(Cursor);

lazy_just!(
      Empty, empty
    ; Sink, sink
    ; Stderr, stderr
    ; Stdin, stdin
    ; Stdout, stdout
);

wrap_ctor!([BufRead] Lines, BufRead::lines);

arbitrary!(Repeat, SMapped<u8, Self>; any_with_smap((), repeat));

arbitrary!(
    [A: BufRead + Arbitrary] Split<A>, SMapped<(A, u8), Self>, A::Parameters;
    args => any_with_smap(product_pack![args, default()], |(a, b)| a.split(b))
);
lift1!(['static + BufRead] Split<A>;
    base => (base, any::<u8>()).prop_map(|(a, b)| a.split(b)));

arbitrary!(
    [A: Read + Arbitrary] Take<A>, SMapped<(A, u64), Self>, A::Parameters;
    args => any_with_smap(product_pack![args, default()], |(a, b)| a.take(b))
);
lift1!(['static + Read] Take<A>;
    base => (base, any::<u64>()).prop_map(|(a, b)| a.take(b)));

#[cfg(feature = "unstable")]
wrap_ctor!([Read] Chars, Read::chars);

arbitrary!(ErrorKind, Union<Just<Self>>;
    Union::new(
    [ NotFound
    , PermissionDenied
    , ConnectionRefused
    , ConnectionReset
    , ConnectionAborted
    , NotConnected
    , AddrInUse
    , AddrNotAvailable
    , BrokenPipe
    , AlreadyExists
    , WouldBlock
    , InvalidInput
    , InvalidData
    , TimedOut
    , WriteZero
    , Interrupted
    , Other
    , UnexpectedEof
    // TODO: watch this type for variant-additions.
    ].into_iter().map(Clone::clone).map(Just))
);

arbitrary!(
    SeekFrom,
    TupleUnion<(
        W<SMapped<u64, SeekFrom>>,
        W<SMapped<i64, SeekFrom>>,
        W<SMapped<i64, SeekFrom>>,
    )>;
    prop_oneof![
        static_map(any::<u64>(), SeekFrom::Start),
        static_map(any::<i64>(), SeekFrom::End),
        static_map(any::<i64>(), SeekFrom::Current)
    ]
);

arbitrary!(Error, SMapped<(ErrorKind, Option<String>), Self>;
    any_with_smap(default(), |(k, os)|
        if let Some(s) = os { Error::new(k, s) } else { k.into() }
    )
);

#[cfg(feature = "unstable")]
arbitrary!(CharsError, SMapped<Option<Error>, Self>;
    any_with_smap(default(), |oe| {
        use std::io::CharsError::*;
        if let Some(e) = oe { Other(e) } else { NotUtf8 }
    })
);

#[cfg(test)]
mod test {
    no_panic_test!(
        buf_reader  => BufReader<Repeat>,
        buf_writer  => BufWriter<Sink>,
        line_writer => LineWriter<Sink>,
        chain       => Chain<Empty, BufReader<Repeat>>,
        cursor      => Cursor<Empty>,
        empty       => Empty,
        sink        => Sink,
        stderr      => Stderr,
        stdin       => Stdin,
        stdout      => Stdout,
        lines       => Lines<Empty>,
        repeat      => Repeat,
        spit        => Split<Cursor<Vec<u8>>>,
        take        => Take<Repeat>,
        error_kind  => ErrorKind,
        seek_from   => SeekFrom,
        error       => Error
    );

    #[cfg(feature = "unstable")]
    no_panic_test!(
        chars       => Chars<Repeat>,
        chars_error => CharsError
    );
}