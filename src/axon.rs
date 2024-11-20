use std::fmt::{Display, Formatter};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

use subxt::tx::Payload;
use crate::api;
use crate::rpc::types::{AxonInfo, PrometheusInfo};

#[derive(Copy, Debug, Clone)]
pub enum AxonProtocol {
    Tcp = 0,
    Udp = 1,
    Other = 4, // Not sure why this is 4, that's just the default
}

impl Default for AxonProtocol {
    fn default() -> Self {
        AxonProtocol::Other
    }
}

impl Display for AxonProtocol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let display = match self {
            AxonProtocol::Tcp => "TCP:0",
            AxonProtocol::Udp => "UDP:1",
            AxonProtocol::Other => "TCP/UDP:4",
        };

        f.write_str(display)
    }
}

pub trait AsAddr {
    fn as_addr(&self) -> SocketAddr;
}

pub trait WithAxonProtocol {
    fn axon_protocol(&self) -> Option<AxonProtocol>;
}

fn neuron_info_as_addr(ip: u128, port: u16, ip_type: u8) -> SocketAddr {
    let ip: IpAddr = if ip_type == 4 {
        Ipv4Addr::from(ip as u32).into()
    } else {
        Ipv6Addr::from(ip).into()
    };

    SocketAddr::new(ip, port)
}

impl AsAddr for AxonInfo {
    fn as_addr(&self) -> SocketAddr {
        neuron_info_as_addr(self.ip, self.port, self.ip_type)
    }
}

impl AsAddr for PrometheusInfo {
    fn as_addr(&self) -> SocketAddr {
        neuron_info_as_addr(self.ip, self.port, self.ip_type)
    }
}

impl WithAxonProtocol for AxonInfo {
    fn axon_protocol(&self) -> Option<AxonProtocol> {
        match self.protocol {
            0 => Some(AxonProtocol::Tcp),
            1 => Some(AxonProtocol::Udp),
            4 => Some(AxonProtocol::Other),
            _ => None,
        }
    }
}

pub fn serve_axon_payload(netuid: u16, address: SocketAddr, protocol: AxonProtocol) -> impl Payload {
    let (ip_addr, ip_type) = match address.ip() {
        IpAddr::V4(addr) => (u128::from(u32::from(addr)), 4u8),
        IpAddr::V6(addr) => (u128::from(addr), 6u8),
    };

    api::tx().subtensor_module().serve_axon(
        netuid,
        1, // version is always 1 in practice
        ip_addr,
        address.port(),
        ip_type,
        protocol as u8,
        0, // placeholder1 unused
        0, // placeholder2 unused
    )
}
