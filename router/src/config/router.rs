use super::backend::UpstreamConfig;
use super::downstream::DownstreamConfig;
use crate::config::filter::FilterConfig;
use crate::config::inet_address::InetAddress;
use serde;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GatewayConfig {
    pub bind_addresses: Vec<InetAddress>,
    pub downstreams: Vec<DownstreamConfig>,
    pub upstreams: Vec<UpstreamConfig>,
    pub filters: Vec<FilterConfig>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RouterConfig {
    pub gateways: Vec<GatewayConfig>,
}
