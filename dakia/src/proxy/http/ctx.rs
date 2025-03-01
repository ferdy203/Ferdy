use std::sync::Arc;

use crate::{config::source_config::GatewayConfig, gateway::state::GatewayState};

pub struct DakiaHttpGatewayCtx {
    pub gateway_state: Arc<GatewayState>,
}

impl DakiaHttpGatewayCtx {
    pub fn new() -> DakiaHttpGatewayCtx {
        DakiaHttpGatewayCtx {
            gateway_state: Arc::new(GatewayState::new(GatewayConfig::default())),
        }
    }
}
