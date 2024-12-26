use std::sync::Arc;

use crate::config::DakiaConfig;
use crate::config::UpstreamNodeConfig;

pub struct DakiaHttpGatewayCtx {
    pub upstream_config: Option<UpstreamNodeConfig>,
    pub config: Arc<DakiaConfig>,
}

impl DakiaHttpGatewayCtx {
    pub fn new() -> DakiaHttpGatewayCtx {
        DakiaHttpGatewayCtx {
            upstream_config: None,
            config: Arc::new(DakiaConfig::default()),
        }
    }
}
