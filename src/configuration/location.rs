use crate::configuration::upstream::Upstream;
use serde;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Location {
    path: String,
    upstreams: Vec<Upstream>,
}
