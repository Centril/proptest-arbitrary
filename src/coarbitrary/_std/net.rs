use coarbitrary::*;

use std::net::*;

coarbitrary_unit!(AddrParseError);

impl CoArbitrary for Ipv4Addr {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.octets());
    }
}

impl CoArbitrary for Ipv6Addr {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.segments());
    }
}

impl CoArbitrary for SocketAddrV4 {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.ip()).nest(&self.port());
    }
}

impl CoArbitrary for SocketAddrV6 {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.ip())
           .nest(&self.port())
           .nest(&self.flowinfo())
           .nest(&self.scope_id());
    }
}

impl CoArbitrary for IpAddr {
    fn coarbitrary(&self, mut var: Perturbable) {
        match *self {
            IpAddr::V4(ref a) => var.variant(0).nest(a),
            IpAddr::V6(ref a) => var.variant(1).nest(a),
        };
    }
}

impl CoArbitrary for SocketAddr {
    fn coarbitrary(&self, mut var: Perturbable) {
        match *self {
            SocketAddr::V4(ref a) => var.variant(0).nest(a),
            SocketAddr::V6(ref a) => var.variant(1).nest(a),
        };
    }
}

impl CoArbitrary for Shutdown {
    fn coarbitrary(&self, mut var: Perturbable) {
        match *self {
            Shutdown::Read => var.variant(0),
            Shutdown::Write => var.variant(1),
            Shutdown::Both => var.variant(2),
        };
    }
}

#[cfg(feature = "unstable")]
impl CoArbitrary for Ipv6MulticastScope {
    fn coarbitrary(&self, mut var: Perturbable) {
        match *self {
            Ipv6MulticastScope::InterfaceLocal => var.variant(0),
            Ipv6MulticastScope::LinkLocal => var.variant(1),
            Ipv6MulticastScope::RealmLocal => var.variant(2),
            Ipv6MulticastScope::AdminLocal => var.variant(3),
            Ipv6MulticastScope::SiteLocal => var.variant(4),
            Ipv6MulticastScope::OrganizationLocal => var.variant(5),
            Ipv6MulticastScope::Global => var.variant(6),
        };
    }
}