use serde;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EndPoint {
    host: String,
    port: u16,
    sni: Option<String>,
    tls: bool,
}

impl EndPoint {
    fn new_tls(host: String, port: u16, sni: String) -> Self {
        EndPoint {
            host,
            port,
            sni: Some(sni),
            tls: true,
        }
    }

    fn new(host: String, port: u16) -> Self {
        EndPoint {
            host,
            port,
            sni: None,
            tls: false,
        }
    }
}
