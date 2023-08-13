use std::net::Ipv6Addr;

use serde::{Deserialize, Serialize};

pub const LOCATION: &str = "/tmp/dhcp6.lease";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PdConfig {
    pub prefix: Ipv6Addr,
    pub len: u8,
    pub preflft: u32,
    pub validlft: u32,
    pub aftr: Option<String>,
}

impl Default for PdConfig {
    fn default() -> Self {
        Self {
            prefix: Ipv6Addr::UNSPECIFIED,
            len: 0,
            preflft: 0,
            validlft: 0,
            aftr: None,
        }
    }
}
