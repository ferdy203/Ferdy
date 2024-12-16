use serde;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Host {
    pub host: String,
    pub port: Option<u16>,
}
