use super::*;
use std::iter::repeat;
use std::str::{ParseBoolError, Utf8Error, from_utf8};

arbitrary!(ParseBoolError; "".parse::<bool>().unwrap_err());

type ELSeq  = W<Just<&'static [u8]>>;
type ELSeqs = TupleUnion<(ELSeq, ELSeq, ELSeq, ELSeq)>;

fn gen_el_seqs() -> ELSeqs {
    prop_oneof![
        Just(&[0xC2]), // None
        Just(&[0x80]), // Some(1)
        Just(&[0xE0, 0xA0, 0x00]), // Some(2)
        Just(&[0xF0, 0x90, 0x80, 0x00]) // Some(3)
    ]
}

arbitrary!(Utf8Error, SFnPtrMap<(StrategyType<'a, u16>, ELSeqs), Utf8Error>;
    static_map((any::<u16>(), gen_el_seqs()), |(vut, elseq)| {
        let v = repeat(b'_').take(vut as usize)
                    .chain(elseq.iter().cloned())
                    .collect::<Vec<u8>>();
        from_utf8(&v).unwrap_err()
    })
);