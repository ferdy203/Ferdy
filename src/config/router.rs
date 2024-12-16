use super::backend::Backend;
use super::downstream::Downstream;
use crate::config::inet_address::InetAddress;
use crate::config::location::Location;
use serde;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Gateway {
    pub bind_addresses: Vec<InetAddress>,
    pub downstreams: Vec<Downstream>,
    pub locations: Vec<Location>,
    pub backends: Vec<Backend>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RouterConfig {
    pub gateways: Vec<Gateway>,
}
