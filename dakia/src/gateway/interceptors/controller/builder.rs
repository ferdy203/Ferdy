use std::sync::Arc;

use crate::{
    config::source_config::InterceptorConfig,
    error::DakiaResult,
    gateway::{
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
        let interceptor = ControllerInterceptor::build(None);
        Ok(Arc::new(interceptor))
    }
}
