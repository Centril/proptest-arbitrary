use super::*;
use std::io::*;
use std::io::ErrorKind::*;

// TODO: IntoInnerError
// Consider: std::io::Initializer

macro_rules! buffer {
    ($type: ident, $bound: path) => {
        arbitrary!(
            [A: Arbitrary<'a> + $bound] $type<A>,
            SMapped<'a, (A, Option<u16>), Self>, A::Parameters;
            args => any_with_smap(args.llift(), |(inner, cap)|
                if let Some(cap) = cap {
                    $type::with_capacity(cap as usize, inner)
                } else {
                    $type::new(inner)
                }
            )
        );
    };
}

buffer!(BufReader,  Read);
buffer!(BufWriter,  Write);
buffer!(LineWriter, Write);

arbitrary!(
    [A: Read + Arbitrary<'a>, B: Read + Arbitrary<'a>] Chain<A, B>,
    SMapped<'a, (A, B), Self>, product_type![A::Parameters, B::Parameters];
    args => any_with_smap(args, |(a, b)| a.chain(b))
);

wrap_ctor!(Cursor);

generator!(
      Empty, empty
    ; Sink, sink
    ; Stderr, stderr
    ; Stdin, stdin
    ; Stdout, stdout
);

wrap_ctor!([BufRead] Lines, BufRead::lines);

arbitrary!(Repeat, SMapped<'a, u8, Self>; any_with_smap((), repeat));

arbitrary!(
    [A: BufRead + Arbitrary<'a>] Split<A>,
    SMapped<'a, (A, u8), Self>, A::Parameters;
    args => any_with_smap(args.llift(), |(a, b)| a.split(b))
);

arbitrary!(
    [A: Read + Arbitrary<'a>] Take<A>,
    SMapped<'a, (A, u64), Self>, A::Parameters;
    args => any_with_smap(args.llift(), |(a, b)| a.take(b))
);

#[cfg(feature = "nightly")]
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
        W<SMapped<'a, u64, SeekFrom>>,
        W<SMapped<'a, i64, SeekFrom>>,
        W<SMapped<'a, i64, SeekFrom>>,
    )>;
    prop_oneof![
        static_map(any::<u64>(), SeekFrom::Start),
        static_map(any::<i64>(), SeekFrom::End),
        static_map(any::<i64>(), SeekFrom::Current)
    ]
);

arbitrary!(Error, SMapped<'a, (ErrorKind, Option<String>), Self>;
    any_with_smap(default(), |(k, os)|
        if let Some(s) = os { Error::new(k, s) } else { k.into() }
    )
);

#[cfg(feature = "nightly")]
arbitrary!(CharsError, SMapped<'a, Option<Error>, Self>;
    any_with_smap(default(), |oe| {
        use std::io::CharsError::*;
        if let Some(e) = oe { Other(e) } else { NotUtf8 }
    })
);