use super::upstream::Upstream;

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
pub struct RoutingPolicy {
    selection_algorithm: SelectionAlgorithm,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Backend {
    pub name: String,
    pub default: bool,
    pub upstreams: Vec<Upstream>,
    pub routing_policy: Option<RoutingPolicy>,
}

impl Backend {
    fn get_upstream_selection_algorithm(&self) -> &SelectionAlgorithm {
        match &self.routing_policy {
            Some(policy) => &policy.selection_algorithm,
            None => &SelectionAlgorithm::RoundRobin,
        }
    }
}
