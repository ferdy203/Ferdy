use std::sync::Arc;

use crate::{
    config::source_config::GatewayConfig,
    shared::{
        config_store,
        pattern_matcher::Pcre2PatternMatcher,
        pattern_registry::{PatternRegistry, PatternRegistryType},
        registry::Registry,
    },
};

use super::DakiaHttpGatewayCtx;
use async_trait::async_trait;
use pingora::{
    prelude::HttpPeer,
    proxy::{ProxyHttp, Session},
    Error,
};

#[derive(Clone)]
pub struct Proxy {
    // ds_pattern_registry: Arc<dyn Registry<Arc<dyn PatternMatcher>> + Send + Sync>,
    ds_host_pattern_registry: PatternRegistryType,
}

impl Proxy {
    pub async fn build(
        gateway_config: &GatewayConfig,
    ) -> Result<Proxy, Box<dyn std::error::Error>> {
        let ds_host_pattern_registry =
            Proxy::build_ds_host_pattern_registry(gateway_config).await?;
        let proxy = Proxy {
            ds_host_pattern_registry,
        };
        Ok(proxy)
    }

    async fn build_ds_host_pattern_registry(
        gateway_config: &GatewayConfig,
    ) -> Result<PatternRegistryType, Box<dyn std::error::Error>> {
        let pr = PatternRegistry {};
        for ds in &gateway_config.downstreams {
            let ds_addr = ds.get_formatted_address();
            let x = Pcre2PatternMatcher::build(&ds_addr)?;
            let _ = pr.register(ds_addr, Arc::new(x)).await;
        }

        Ok(Arc::new(pr))
    }
}

#[async_trait]
impl ProxyHttp for Proxy {
    type CTX = DakiaHttpGatewayCtx;
    fn new_ctx(&self) -> Self::CTX {
        DakiaHttpGatewayCtx::new()
    }

    async fn early_request_filter(
        &self,
        _session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> Result<(), Box<Error>> {
        // update config into context
        let c = config_store::get().await;
        _ctx.config = c;

        Ok(())
    }

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>, Box<Error>> {
        let addr = ("127.0.0.1", 3000);

        let peer = Box::new(HttpPeer::new(addr, false, "one.one.one.one".to_string()));
        Ok(peer)
    }
}
