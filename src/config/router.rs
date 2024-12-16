use super::backend::Backend;
use super::downstream::DownstreamConfig;
use crate::config::inet_address::InetAddress;
use crate::config::location::Route;
use serde;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GatewayConfig {
    pub bind_addresses: Vec<InetAddress>,
    pub downstreams: Vec<DownstreamConfig>,
    pub backends: Vec<Backend>,
    pub routes: Vec<Route>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RouterConfig {
    pub gateways: Vec<GatewayConfig>,
}
