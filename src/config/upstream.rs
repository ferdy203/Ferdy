use serde;

use super::inet_address::InetAddress;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UpstreamNodeConfig {
    pub address: InetAddress,
    pub tls: bool,
    pub sni: Option<String>,
    pub weight: Option<u16>,
}
