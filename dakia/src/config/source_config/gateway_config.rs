use super::DownstreamConfig;
use super::InetAddress;
use super::RouterConfig;
use super::UpstreamConfig;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GatewayConfig {
    // TODO: add type = HTTP, TCP, SMTP, etc
    pub bind_addresses: Vec<InetAddress>,
    pub downstreams: Vec<DownstreamConfig>,
    pub upstreams: Vec<UpstreamConfig>,

    #[serde(default)]
    pub routers: Vec<RouterConfig>,
}
