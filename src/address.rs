use core::net::{IpAddr, Ipv4Addr, SocketAddr};

use tokio::net::ToSocketAddrs;

pub fn address(port: u16) -> impl ToSocketAddrs {
    SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), port)
}
