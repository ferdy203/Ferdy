use crate::configuration::end_point::EndPoint;
use serde;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Upstream {
    end_point: EndPoint,
    weight: Option<u16>,
}
