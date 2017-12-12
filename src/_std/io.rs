use super::*;
use std::io::*;
use std::io::ErrorKind::*;
use frunk_core::hlist::LiftInto;
use proptest::strategy::{Just, Union, TupleUnion};

arbitrary_for!(
    [A: Read + Arbitrary<'a>] BufReader<A>,
    SMapped<'a, (A, Option<u16>), Self>, A::Parameters,
    args => any_with_smap(args.lift_into(), |(inner, cap)|
        if let Some(cap) = cap {
            BufReader::with_capacity(cap as usize, inner)
        } else {
            BufReader::new(inner)
        }
    )
);
arbitrary_for!(
    [A: Write + Arbitrary<'a>] BufWriter<A>,
    SMapped<'a, (A, Option<u16>), Self>, A::Parameters,
    args => any_with_smap(args.lift_into(), |(inner, cap)|
        if let Some(cap) = cap {
            BufWriter::with_capacity(cap as usize, inner)
        } else {
            BufWriter::new(inner)
        }
    )
);
arbitrary_for!(
    [A: Read + Arbitrary<'a>, B: Read + Arbitrary<'a>] Chain<A, B>,
    SMapped<'a, (A, B), Self>, Hlist![A::Parameters, B::Parameters],
    args => any_with_smap(args, |(a, b)| a.chain(b))
);
impl_wrap_gen!([] Cursor);
gen_strat!(
      Empty, empty
    ; Sink, sink
    ; Stderr, stderr
    ; Stdin, stdin
    ; Stdout, stdout
);
// TODO: Error
// TODO: IntoInnerError
arbitrary_for!(
    [A: Write + Arbitrary<'a>] LineWriter<A>,
    SMapped<'a, (A, Option<u16>), Self>, A::Parameters,
    args => any_with_smap(args.lift_into(), |(inner, cap)|
        if let Some(cap) = cap {
            LineWriter::with_capacity(cap as usize, inner)
        } else {
            LineWriter::new(inner)
        }
    )
);
impl_wrap_gen!([BufRead] Lines, BufRead::lines);
impl_arbitrary!(Repeat, SMapped<'a, u8, Self>, any_with_smap((), repeat));
arbitrary_for!(
    [A: BufRead + Arbitrary<'a>] Split<A>,
    SMapped<'a, (A, u8), Self>, A::Parameters,
    args => any_with_smap(args.lift_into(), |(a, b)| a.split(b))
);
arbitrary_for!(
    [A: Read + Arbitrary<'a>] Take<A>,
    SMapped<'a, (A, u64), Self>, A::Parameters,
    args => any_with_smap(args.lift_into(), |(a, b)| a.take(b))
);
impl_wrap_gen!([Read] Chars, Read::chars);
// Consider: std::io::Initializer
impl_arbitrary!(ErrorKind, Union<Just<Self>>,
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
impl_arbitrary!(
    SeekFrom,
    TupleUnion<(
        W<SMapped<'a, u64, SeekFrom>>,
        W<SMapped<'a, i64, SeekFrom>>,
        W<SMapped<'a, i64, SeekFrom>>,
    )>,
    prop_oneof![
        static_map(any::<u64>(), SeekFrom::Start),
        static_map(any::<i64>(), SeekFrom::End),
        static_map(any::<i64>(), SeekFrom::Current)
    ]
);
impl_arbitrary!(Error, SMapped<'a, (ErrorKind, Option<String>), Self>,
    any_with_smap(Default::default(), |(k, os)|
        if let Some(s) = os { Error::new(k, s) } else { k.into() }
    )
);
impl_arbitrary!(CharsError, SMapped<'a, Option<Error>, Self>,
    any_with_smap(Default::default(), |oe| {
        use std::io::CharsError::*;
        if let Some(e) = oe { Other(e) } else { NotUtf8 }
    })
);