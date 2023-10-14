use std::net::Ipv6Addr;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

pub const LOCATION: &str = "/data/dhcp6.lease";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PdConfig {
    pub timestamp: SystemTime,
    pub server: Ipv6Addr,
    pub server_id: Vec<u8>,
    pub t1: u32,
    pub t2: u32,
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
            timestamp: SystemTime::UNIX_EPOCH,
            server: Ipv6Addr::UNSPECIFIED,
            server_id: Vec::default(),
            t1: 0,
            t2: 0,
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
