use crate::{
    config::source_config::GatewayConfig,
    error::DakiaResult,
    proxy::http::{builder, lb},
    shared::pattern_registry::PatternRegistryType,
};
use arc_swap::ArcSwap;
use std::sync::Arc;

#[derive(Clone)]
pub struct GatewayState {
    _version: i64,
    gateway_config: GatewayConfig,
    ds_host_pattern_registry: PatternRegistryType,
    lb_registry: lb::LbRegistryType,
}

impl GatewayState {
    pub fn build(
        gateway_config: GatewayConfig,
        ds_host_pattern_registry: PatternRegistryType,
        lb_registry: lb::LbRegistryType,
    ) -> Self {
        Self {
            _version: 0,
            gateway_config,
            ds_host_pattern_registry,
            lb_registry,
        }
    }

    pub fn get_gateway_config(&self) -> &GatewayConfig {
        &self.gateway_config
    }

    pub fn get_pattern_registry(&self) -> &PatternRegistryType {
        &self.ds_host_pattern_registry
    }

    pub fn get_lb_registry(&self) -> &lb::LbRegistryType {
        &self.lb_registry
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

pub async fn build_gateway_state(gateway_config: GatewayConfig) -> DakiaResult<GatewayState> {
    let ds_host_pattern_registry = builder::build_ds_host_pattern_registry(&gateway_config).await?;
    let lb_registry = builder::build_lb_registry(&gateway_config).await?;
    let gateway_state = GatewayState::build(gateway_config, ds_host_pattern_registry, lb_registry);
    Ok(gateway_state)
}
