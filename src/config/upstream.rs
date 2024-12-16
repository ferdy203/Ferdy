use serde;

use super::inet_address::InetAddress;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UpstreamConfig {
    pub address: InetAddress,
    pub tls: bool,
    pub sni: Option<String>,
    pub weight: Option<u16>,
}

// TODO: add support for weight
impl UpstreamConfig {
    pub fn _new(inet_address: InetAddress) -> UpstreamConfig {
        UpstreamConfig {
            address: inet_address,
            tls: false,
            sni: None,
            weight: None,
        }
    }

    pub fn _new_tls(address: InetAddress, sni: String) -> UpstreamConfig {
        UpstreamConfig {
            address,
            tls: true,
            sni: Some(sni),
            weight: None,
        }
    }
}
