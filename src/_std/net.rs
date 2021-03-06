//! Arbitrary implementations for `std::net`.

use super::*;
use std::net::*;

// TODO: Can we design a workable semantic for PBT wrt. actual networking
// connections?

arbitrary!(AddrParseError; "".parse::<Ipv4Addr>().unwrap_err());

arbitrary!(Ipv4Addr,
    TupleUnion<(W<Just<Self>>, W<Just<Self>>, W<FMapped<u32, Self>>)>;
    prop_oneof![
        1  => Just(Self::new(0, 0, 0, 0)),
        4  => Just(Self::new(127, 0, 0, 1)),
        10 => any_sinto::<u32, _>()
    ]
);

arbitrary!(Ipv6Addr,
    TupleUnion<(W<SMapped<Ipv4Addr, Self>>, W<FMapped<[u16; 8], Self>>)>;
    prop_oneof![
        2 => any_with_smap((), |ip| ip.to_ipv6_mapped()),
        1 => any_sinto::<[u16; 8], _>()
    ]
);

arbitrary!(SocketAddrV4, SMapped<(Ipv4Addr, u16), Self>;
    static_map(any::<(Ipv4Addr, u16)>(), |(a, b)| Self::new(a, b))
);

arbitrary!(SocketAddrV6, SMapped<(Ipv6Addr, u16, u32, u32), Self>;
    static_map(any::<(Ipv6Addr, u16, u32, u32)>(),
        |(a, b, c, d)| Self::new(a, b, c, d))
);

arbitrary!(IpAddr,
    TupleUnion<(W<FMapped<Ipv4Addr, Self>>,
                W<FMapped<Ipv6Addr, Self>>)>;
    prop_oneof![any_sinto::<Ipv4Addr, _>(), any_sinto::<Ipv6Addr, _>()]
);

arbitrary!(Shutdown,
    TupleUnion<(W<Just<Self>>, W<Just<Self>>, W<Just<Self>>)>;
    {
        use std::net::Shutdown::*;
        prop_oneof![Just(Both), Just(Read), Just(Write)]
    }
);
arbitrary!(SocketAddr,
    TupleUnion<(W<FMapped<SocketAddrV4, Self>>,
                W<FMapped<SocketAddrV6, Self>>)>;
    prop_oneof![any_sinto::<SocketAddrV4, _>(), any_sinto::<SocketAddrV6, _>()]
);

#[cfg(feature = "unstable")]
arbitrary!(Ipv6MulticastScope,
    TupleUnion<( W<Just<Self>>, W<Just<Self>>, W<Just<Self>>
               , W<Just<Self>>, W<Just<Self>>, W<Just<Self>>, W<Just<Self>>)>;
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

#[cfg(test)]
mod test {
    no_panic_test!(
        addr_parse_error => AddrParseError,
        ipv4_addr => Ipv4Addr,
        ipv6_addr => Ipv6Addr,
        socket_addr_v4 => SocketAddrV4,
        socket_addr_v6 => SocketAddrV6,
        ip_addr => IpAddr,
        shutdown => Shutdown,
        socket_addr => SocketAddr
    );

    #[cfg(feature = "unstable")]
    no_panic_test!(
        ipv6_multicast_scope => Ipv6MulticastScope
    );
}