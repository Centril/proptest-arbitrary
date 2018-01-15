use std::net::*;

coarbitrary_unit!(AddrParseError);

coarbitrary!(Ipv4Addr; self, var => var.nest(&self.octets()));
coarbitrary!(Ipv6Addr; self, var => var.nest(&self.segments()));
coarbitrary!(SocketAddrV4; self, var => var.nest(&self.ip()).nest(&self.port()));
coarbitrary!(SocketAddrV6; self, var =>
    var.nest(&self.ip())
       .nest(&self.port())
       .nest(&self.flowinfo())
       .nest(&self.scope_id())
);
coarbitrary!(IpAddr; self, var => match *self {
    IpAddr::V4(ref a) => var.variant(0).nest(a),
    IpAddr::V6(ref a) => var.variant(1).nest(a),
});
coarbitrary!(SocketAddr; self, var => match *self {
    SocketAddr::V4(ref a) => var.variant(0).nest(a),
    SocketAddr::V6(ref a) => var.variant(1).nest(a),
});
coarbitrary!(Shutdown; self, var => match *self {
    Shutdown::Read => var.variant(0),
    Shutdown::Write => var.variant(1),
    Shutdown::Both => var.variant(2),
});

#[cfg(feature = "unstable")]
coarbitrary!(Ipv6MulticastScope; self, var => match *self {
    Ipv6MulticastScope::InterfaceLocal => var.variant(0),
    Ipv6MulticastScope::LinkLocal => var.variant(1),
    Ipv6MulticastScope::RealmLocal => var.variant(2),
    Ipv6MulticastScope::AdminLocal => var.variant(3),
    Ipv6MulticastScope::SiteLocal => var.variant(4),
    Ipv6MulticastScope::OrganizationLocal => var.variant(5),
    Ipv6MulticastScope::Global => var.variant(6),
});