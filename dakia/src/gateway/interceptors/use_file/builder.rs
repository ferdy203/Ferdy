use std::{collections::HashMap, sync::Arc};

use crate::{
    config::source_config::InterceptorConfig,
    error::{DakiaError, DakiaResult},
    gateway::{
        interceptor::{Interceptor, InterceptorName},
        interceptor_builder::{self, InterceptorBuilder},
    },
    qe::query,
};

use super::UseFileInterceptor;

pub struct UseFileInterceptorBuilder {}

impl Default for UseFileInterceptorBuilder {
    fn default() -> Self {
        Self {}
    }
}

impl UseFileInterceptorBuilder {
    fn extract_root(&self, interceptor_config: &InterceptorConfig) -> DakiaResult<String> {
        let config = interceptor_config.config.as_ref().expect(
            format!(
                "config requried for {:?} interceptor",
                InterceptorName::UseFile
            )
            .as_str(),
        );
        let root_val = config.get("root").expect(
            format!(
                "root value is undefined in config of {:?} interceptor",
                InterceptorName::UseFile,
            )
            .as_str(),
        );

        let root = match root_val {
            query::Value::Scaler(scaler) => match scaler {
                query::Scaler::String(root) => Ok(root),
                _ => Err(DakiaError::i_explain(format!(
                    "root value must be an string config of {:?} interceptor",
                    InterceptorName::UseFile,
                ))),
            },
            query::Value::Composite(_) => Err(DakiaError::i_explain(format!(
                "root value must be an string config of {:?} interceptor",
                InterceptorName::UseFile,
            ))),
        }?;

        Ok(root.clone())
    }
}

impl InterceptorBuilder for UseFileInterceptorBuilder {
    fn build(&self, interceptor_config: InterceptorConfig) -> DakiaResult<Arc<dyn Interceptor>> {
        let root = self.extract_root(&interceptor_config)?;
        let header_buffers = match &interceptor_config.intercept {
            Some(query) => interceptor_builder::utils::extract_headers(query)?,
            None => (HashMap::new(), HashMap::new()),
        };

        let interceptor = UseFileInterceptor::build(root, header_buffers);
        Ok(Arc::new(interceptor))
    }
}
