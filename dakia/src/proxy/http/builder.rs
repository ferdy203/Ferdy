use std::sync::Arc;

use crate::{
    config::source_config::GatewayConfig,
    shared::{
        pattern_matcher::Pcre2PatternMatcher,
        pattern_registry::{PatternRegistry, PatternRegistryType},
        registry::Registry,
    },
};

pub async fn build_ds_host_pattern_registry(
    gateway_config: &GatewayConfig,
) -> Result<PatternRegistryType, Box<dyn std::error::Error>> {
    let pr = PatternRegistry::build();
    for ds in &gateway_config.downstreams {
        let ds_addr = ds.get_formatted_address();
        let x = Pcre2PatternMatcher::build(&ds_addr)?;
        let _ = pr.register(ds_addr, Arc::new(x)).await;
    }

    Ok(Arc::new(pr))
}
