use std::sync::Arc;

use crate::{
    config::source_config::InterceptorConfig,
    error::DakiaResult,
    gateway::{
        interceptor::Interceptor, interceptor_builder::InterceptorBuilder,
        interceptors::version::VersionInterceptor,
    },
};

pub struct VersionBuilder {}

impl Default for VersionBuilder {
    fn default() -> Self {
        Self {}
    }
}

impl InterceptorBuilder for VersionBuilder {
    fn build(&self, interceptor_config: InterceptorConfig) -> DakiaResult<Arc<dyn Interceptor>> {
        let interceptor = VersionInterceptor {};
        Ok(Arc::new(interceptor))
    }
}
