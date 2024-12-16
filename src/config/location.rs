use serde;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Route {
    // TODO: add support for wild card paths
    // TODO: add support for regex path matching
    pub pattern: String,
    pub backend: String,
}
