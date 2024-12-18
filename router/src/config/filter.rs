use serde;

use super::pattern::PatternType;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RouteConfig {
    // TODO: add support for regex path matching
    pub route: String,
    pub route_type: PatternType,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Router {
    pub route: Option<RouteConfig>,
    pub upstream: String,
}
// TODO: add a method to take argument of request and verify if it matches not
