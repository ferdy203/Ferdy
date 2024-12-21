#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RouterConfig {
    filter: Option<i32>,
    upstream: String,
}
