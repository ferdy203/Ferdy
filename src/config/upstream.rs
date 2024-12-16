use serde;

use super::inet_address::InetAddress;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Upstream {
    pub inet_address: InetAddress,
    pub sni: Option<String>,
    pub tls: bool,
    pub weight: Option<u16>,
}

// TODO: add support for weight
impl Upstream {
    pub fn new(inet_address: InetAddress) -> Upstream {
        Upstream {
            inet_address,
            tls: false,
            sni: None,
            weight: None,
        }
    }

    pub fn new_tls(inet_address: InetAddress, sni: String) -> Upstream {
        Upstream {
            inet_address,
            tls: true,
            sni: Some(sni),
            weight: None,
        }
    }
}
