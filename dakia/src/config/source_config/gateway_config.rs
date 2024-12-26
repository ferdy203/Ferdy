use super::DownstreamConfig;
use super::InetAddress;
use super::RouterConfig;
use super::UpstreamConfig;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GatewayConfig {
    pub bind_address: Vec<InetAddress>,
    pub downstreams: Vec<DownstreamConfig>,
    pub upstreams: Vec<UpstreamConfig>,
    pub routers: Vec<RouterConfig>,
}
