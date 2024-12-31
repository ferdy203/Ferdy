use serde::{Deserialize, Serialize};

use crate::qe::query::Query;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouterConfig {
    pub filter: Option<Query>,
    pub upstream: String,
}
