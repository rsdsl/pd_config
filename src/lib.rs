use std::net::Ipv6Addr;

use serde::{Deserialize, Serialize};

pub const LOCATION: &str = "/data/dhcp6.lease";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PdConfig {
    pub prefix: Ipv6Addr,
    pub len: u8,
    pub preflft: u32,
    pub validlft: u32,
    pub dns1: Ipv6Addr,
    pub dns2: Ipv6Addr,
    pub aftr: Option<String>,
}

impl Default for PdConfig {
    fn default() -> Self {
        Self {
            prefix: Ipv6Addr::UNSPECIFIED,
            len: 0,
            preflft: 0,
            validlft: 0,
            dns1: Ipv6Addr::UNSPECIFIED,
            dns2: Ipv6Addr::UNSPECIFIED,
            aftr: None,
        }
    }
}
