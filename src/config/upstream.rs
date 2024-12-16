use serde;

use super::inet_address::InetAddress;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Upstream {
    pub address: InetAddress,
    pub tls: bool,
    pub sni: Option<String>,
    pub weight: Option<u16>,
}

// TODO: add support for weight
impl Upstream {
    pub fn new(inet_address: InetAddress) -> Upstream {
        Upstream {
            address: inet_address,
            tls: false,
            sni: None,
            weight: None,
        }
    }

    pub fn new_tls(address: InetAddress, sni: String) -> Upstream {
        Upstream {
            address,
            tls: true,
            sni: Some(sni),
            weight: None,
        }
    }
}
