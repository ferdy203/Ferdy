use crate::{gateway::interceptor::InterceptorName, qe::query::Query};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InterceptorConfig {
    pub name: InterceptorName,
    pub enabled: bool,
    pub filter: Option<Query>,
    pub config: Option<Query>,
    pub intercept: Option<Query>,
}
