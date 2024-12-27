use serde;

use super::inet_address::InetAddress;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeSelectionAlgorithm {
    RoundRobin,
    Weighted,
    LeastConnection,
    IpHash,
    UrlHash,
    Random,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TrafficDistributionPolicy {
    node_selection_algorithm: NodeSelectionAlgorithm,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UpstreamNodeConfig {
    pub address: InetAddress,
    pub tls: bool,
    pub sni: Option<String>,
    pub weight: Option<u16>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UpstreamConfig {
    pub name: String,
    pub default: bool,
    pub upstream_nodes: Vec<UpstreamNodeConfig>,
    pub traffic_distribution_policy: Option<TrafficDistributionPolicy>,
}
