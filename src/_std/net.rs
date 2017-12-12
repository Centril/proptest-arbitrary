use super::*;
use std::net::*;
use std::net::Shutdown::*;
use proptest::strategy::{Just, TupleUnion};

// TODO: Can we design a workable semantic for PBT wrt. actual networking
// connections?

impl_just!(AddrParseError, "".parse::<Ipv4Addr>().unwrap_err());

impl_arbitrary!(Ipv4Addr,
    TupleUnion<(
        W<Just<Self>>,
        W<Just<Self>>,
        W<FMapped<'a, u32, Self>>,
    )>,
    prop_oneof![
        1  => Just(Self::new(0, 0, 0, 0)),
        4  => Just(Self::new(127, 0, 0, 1)),
        10 => any_sinto::<u32, _>()
    ]
);
impl_arbitrary!(Ipv6Addr,
    TupleUnion<(W<SMapped<'a, Ipv4Addr, Self>>, W<FMapped<'a, [u16; 8], Self>>)>,
    prop_oneof![
        2 => any_with_smap((), |ip| ip.to_ipv6_mapped()),
        1 => any_sinto::<[u16; 8], _>()
    ]
);
impl_arbitrary!(SocketAddrV4, SMapped<'a, (Ipv4Addr, u16), Self>,
    static_map(any::<(Ipv4Addr, u16)>(), |(a, b)| Self::new(a, b))
);
impl_arbitrary!(SocketAddrV6, SMapped<'a, (Ipv6Addr, u16, u32, u32), Self>,
    static_map(any::<(Ipv6Addr, u16, u32, u32)>(),
        |(a, b, c, d)| Self::new(a, b, c, d))
);
impl_arbitrary!(IpAddr,
    TupleUnion<(W<FMapped<'a, Ipv4Addr, Self>>,
                W<FMapped<'a, Ipv6Addr, Self>>)>,
    prop_oneof![any_sinto::<Ipv4Addr, _>(), any_sinto::<Ipv6Addr, _>()]
);
impl_arbitrary!(Shutdown,
    TupleUnion<(W<Just<Self>>, W<Just<Self>>, W<Just<Self>>)>,
    prop_oneof![Just(Both), Just(Read), Just(Write)]
);
impl_arbitrary!(SocketAddr,
    TupleUnion<(W<FMapped<'a, SocketAddrV4, Self>>,
                W<FMapped<'a, SocketAddrV6, Self>>)>,
    prop_oneof![any_sinto::<SocketAddrV4, _>(), any_sinto::<SocketAddrV6, _>()]
);
impl_arbitrary!(Ipv6MulticastScope,
    TupleUnion<( W<Just<Self>>, W<Just<Self>>, W<Just<Self>>
               , W<Just<Self>>, W<Just<Self>>, W<Just<Self>>, W<Just<Self>>)>,
    {
        use std::net::Ipv6MulticastScope::*;
        prop_oneof![
            Just(InterfaceLocal),
            Just(LinkLocal),
            Just(RealmLocal),
            Just(AdminLocal),
            Just(SiteLocal),
            Just(OrganizationLocal),
            Just(Global),
        ]
    }
);