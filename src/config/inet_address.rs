use serde;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InetAddress {
    host: String,
    port: Option<u16>,
}
