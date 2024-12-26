#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RouterConfig {
    pub filter: Option<i32>,
    pub upstream: String,
}
