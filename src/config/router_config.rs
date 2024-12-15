use crate::config::inet_address::InetAddress;
use crate::config::location::Location;
use serde;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RouterConfig {
    pub listen: Vec<InetAddress>,
    pub hosts: Vec<InetAddress>,
    pub locations: Vec<Location>,
}
