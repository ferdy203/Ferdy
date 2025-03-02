use std::sync::Arc;

use crate::{
    config::source_config::InterceptorConfig,
    error::DakiaResult,
    gateway::{
        interceptor::Interceptor, interceptor_builder::InterceptorBuilder,
        interceptors::server::ServerInterceptor,
    },
};

pub struct ServerInterceptorBuilder {}

impl Default for ServerInterceptorBuilder {
    fn default() -> Self {
        Self {}
    }
}

impl InterceptorBuilder for ServerInterceptorBuilder {
    fn build(&self, interceptor_config: InterceptorConfig) -> DakiaResult<Arc<dyn Interceptor>> {
        let interceptor = ServerInterceptor {};
        Ok(Arc::new(interceptor))
    }
}
