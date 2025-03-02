use crate::{
    config::source_config::GatewayConfig,
    error::{DakiaError, DakiaResult},
    proxy::http::{builder, lb},
    shared::pattern_registry::PatternRegistryType,
};
use arc_swap::ArcSwap;
use std::sync::Arc;

use super::{interceptor::Interceptor, interceptor_builder::InterceptorBuilderRegistry};

#[derive(Clone)]
pub struct GatewayState {
    _version: i64,
    gateway_config: GatewayConfig,
    ds_host_pattern_registry: PatternRegistryType,
    lb_registry: lb::LbRegistryType,
    interceptor_builder_registry: InterceptorBuilderRegistry,
    interceptors: Vec<Arc<dyn Interceptor>>,
}

impl GatewayState {
    pub fn build(
        gateway_config: GatewayConfig,
        ds_host_pattern_registry: PatternRegistryType,
        lb_registry: lb::LbRegistryType,
        interceptor_builder_registry: InterceptorBuilderRegistry,
        interceptors: Vec<Arc<dyn Interceptor>>,
    ) -> Self {
        Self {
            _version: 0,
            gateway_config,
            ds_host_pattern_registry,
            lb_registry,
            interceptor_builder_registry,
            interceptors,
        }
    }

    pub fn gateway_config(&self) -> &GatewayConfig {
        &self.gateway_config
    }

    pub fn pattern_registry(&self) -> &PatternRegistryType {
        &self.ds_host_pattern_registry
    }

    pub fn lb_registry(&self) -> &lb::LbRegistryType {
        &self.lb_registry
    }

    pub fn interceptors(&self) -> &Vec<Arc<dyn Interceptor>> {
        &self.interceptors
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
    pub fn update_state(&self, new_state: GatewayState) -> () {
        self.state.swap(Arc::new(new_state));
    }

    pub fn get_state(&self) -> Arc<GatewayState> {
        self.state.load_full()
    }

    pub fn get_inner(&self) -> GatewayState {
        let arc_config = self.get_state().clone();
        (*arc_config).clone()
    }
}

pub async fn build_gateway_state(gateway_config: GatewayConfig) -> DakiaResult<GatewayState> {
    let ds_host_pattern_registry = builder::build_ds_host_pattern_registry(&gateway_config).await?;
    let lb_registry = builder::build_lb_registry(&gateway_config).await?;

    let interceptor_builder_registry = InterceptorBuilderRegistry::build();
    let interceptors = build_interceptors(&gateway_config, &interceptor_builder_registry)?;

    let gateway_state = GatewayState::build(
        gateway_config,
        ds_host_pattern_registry,
        lb_registry,
        interceptor_builder_registry,
        interceptors,
    );

    Ok(gateway_state)
}

fn build_interceptors(
    gateway_config: &GatewayConfig,
    interceptor_builder_registry: &InterceptorBuilderRegistry,
) -> DakiaResult<Vec<Arc<dyn Interceptor>>> {
    let mut interceptors: Vec<Arc<dyn Interceptor>> = vec![];

    for interceptor_config in &gateway_config.interceptors {
        if !interceptor_config.enabled {
            continue;
        }

        let interceptor_name = &interceptor_config.name;
        let builder = interceptor_builder_registry.registry.get(interceptor_name);
        let interceptor = match builder {
            Some(builder) => builder.build(interceptor_config.clone())?,
            None => {
                return Err(DakiaError::i_explain(format!(
                    "Invalid interceptor name {:?}. No such interceptor exists",
                    interceptor_name.as_str()
                )))
            }
        };
        interceptors.push(interceptor);
    }

    Ok(interceptors)
}
