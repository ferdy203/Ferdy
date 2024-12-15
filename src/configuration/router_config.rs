use crate::configuration::end_point;
use crate::configuration::location;
use serde;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RouterConfig {
    pub listen: Vec<end_point::EndPoint>,
    pub hosts: Vec<String>,
    pub locations: Vec<location::Location>,
}
