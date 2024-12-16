use serde;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Location {
    // TODO: add support for wild card paths
    // TODO: add support for regex path matching
    pub path: String,
    pub backend: String,
}
