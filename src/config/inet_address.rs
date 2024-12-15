use serde;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InetAddress {
    pub host: String,
    pub port: u16,
}
