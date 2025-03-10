use std::sync::Arc;

use crate::{
    config::source_config::InterceptorConfig,
    error::DakiaResult,
    gateway::{
        filter::Filter,
        interceptor::{HeaderBuffers, Interceptor},
        interceptor_builder::InterceptorBuilder,
    },
};

use super::ControllerInterceptor;

pub struct ControllerInterceptorBuilder {}

impl Default for ControllerInterceptorBuilder {
    fn default() -> Self {
        Self {}
    }
}

impl InterceptorBuilder for ControllerInterceptorBuilder {
    fn build(
        &self,
        _interceptor_config: InterceptorConfig,
        _header_buffers: HeaderBuffers,
    ) -> DakiaResult<Arc<dyn Interceptor>> {
        let filter = match &_interceptor_config.filter {
            Some(filter_config) => {
                let filter = Filter::try_from(filter_config)?;
                Some(filter)
            }
            None => None,
        };
        let interceptor = ControllerInterceptor::build(filter);
        Ok(Arc::new(interceptor))
    }
}
