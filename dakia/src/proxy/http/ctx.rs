use std::sync::Arc;

use crate::gateway::state::GatewayState;

pub struct DakiaHttpGatewayCtx {
    pub gateway_state: Arc<GatewayState>,
}

impl DakiaHttpGatewayCtx {
    pub fn new(gateway_state: Arc<GatewayState>) -> DakiaHttpGatewayCtx {
        DakiaHttpGatewayCtx { gateway_state }
    }
}
