use crate::config::source_config::GatewayConfig;
use arc_swap::ArcSwap;
use std::sync::Arc;

#[derive(Clone)]
pub struct GatewayState {
    _version: i64,
    dakia_config: GatewayConfig,
}

impl GatewayState {
    pub fn new(dakia_config: GatewayConfig) -> Self {
        Self {
            _version: 0,
            dakia_config,
        }
    }

    pub fn get_gateway_config(&self) -> &GatewayConfig {
        &self.dakia_config
    }
}
pub struct GatewayStateStore {
    state: ArcSwap<GatewayState>,
}

impl GatewayStateStore {
    pub fn new(state: GatewayState) -> Self {
        Self {
            state: ArcSwap::new(Arc::new(state)),
        }
    }
}

impl GatewayStateStore {
    pub fn update_state(&mut self, new_state: GatewayState) -> () {
        self.state.swap(Arc::new(new_state));
    }

    pub fn get_state(&self) -> Arc<GatewayState> {
        self.state.load_full()
    }

    pub fn get_inner(&self) -> GatewayState {
        let arc_config = self.get_state();
        (*arc_config).clone()
    }
}
