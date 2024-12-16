use super::upstream::UpstreamConfig;

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
pub struct Backend {
    pub name: String,
    pub default: bool,
    pub upstreams: Vec<UpstreamConfig>,
    pub traffic_distribution_policy: Option<TrafficDistributionPolicy>,
}

impl Backend {
    pub fn get_upstream_config(&self) -> Option<&UpstreamConfig> {
        // match &self.traffic_distribution_policy {
        //     Some(policy) => &policy.selection_algorithm,
        //     None => &SelectionAlgorithm::RoundRobin,
        // }
        self.upstreams.get(0)
    }
}
