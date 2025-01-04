use std::sync::Arc;

use crate::config::DakiaConfig;

pub struct DakiaHttpGatewayCtx {
    pub config: Arc<DakiaConfig>,
}

impl DakiaHttpGatewayCtx {
    pub fn new() -> DakiaHttpGatewayCtx {
        DakiaHttpGatewayCtx {
            config: Arc::new(DakiaConfig::default()),
        }
    }
}
