use super::upstream::UpstreamNodeConfig;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum SelectionAlgorithm {
    RoundRobin,
    Weighted,
    LeastConnection,
    IpHash,
    UrlHash,
    Random,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TrafficDistributionPolicy {
    selection_algorithm: SelectionAlgorithm,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UpstreamConfig {
    pub name: String,
    pub default: bool,
    pub upstreams: Vec<UpstreamNodeConfig>,
    pub traffic_distribution_policy: Option<TrafficDistributionPolicy>,
}

impl UpstreamConfig {
    pub fn get_upstream_node_config(&self) -> Option<&UpstreamNodeConfig> {
        self.upstreams.get(0)
    }
}
