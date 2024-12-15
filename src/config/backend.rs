use super::upstream::Upstream;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum UpstreamSelectionAlgorithm {
    RoundRobin,
    Weighted,
    LeastConnection,
    IpHash,
    UrlHash,
    Random,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RoutingPolicy {
    upstream_selection_algorithm: UpstreamSelectionAlgorithm,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Backend {
    name: String,
    default: bool,
    upstreams: Vec<Upstream>,
    routing_policy: Option<RoutingPolicy>,
}

impl Backend {
    fn get_upstream_selection_algorithm(&self) -> &UpstreamSelectionAlgorithm {
        match &self.routing_policy {
            Some(policy) => &policy.upstream_selection_algorithm,
            None => &UpstreamSelectionAlgorithm::RoundRobin,
        }
    }
}
