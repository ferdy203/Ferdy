use crate::{
    gateway::interceptor::{HookMask, InterceptorName, PhaseMask},
    qe::query::Query,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InterceptorConfig {
    pub name: InterceptorName,
    pub enabled: bool,
    pub phase_mask: Option<PhaseMask>,
    pub hook_mast: Option<HookMask>,
    pub filter: Option<Query>,
    pub config: Option<Query>,
    pub intercept: Option<Query>,
}
