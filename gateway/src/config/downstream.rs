use serde;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DownstreamConfig {
    pub host: String,
    pub port: Option<u16>,
}
