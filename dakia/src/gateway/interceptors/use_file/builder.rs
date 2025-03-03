use std::sync::Arc;

use crate::{
    config::source_config::InterceptorConfig,
    error::DakiaResult,
    gateway::{interceptor::Interceptor, interceptor_builder::InterceptorBuilder},
};

use super::UseFileInterceptor;

pub struct UseFileInterceptorBuilder {}

impl Default for UseFileInterceptorBuilder {
    fn default() -> Self {
        Self {}
    }
}

impl InterceptorBuilder for UseFileInterceptorBuilder {
    fn build(&self, interceptor_config: InterceptorConfig) -> DakiaResult<Arc<dyn Interceptor>> {
        let interceptor = UseFileInterceptor {};
        Ok(Arc::new(interceptor))
    }
}
