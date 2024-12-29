use crate::{config::source_config::GatewayConfig, error::DakiaError, globals::config_store};

use super::DakiaHttpGatewayCtx;
use async_trait::async_trait;
use pingora::{
    prelude::HttpPeer,
    proxy::{ProxyHttp, Session},
    Error,
};

pub struct Proxy {}

impl Proxy {
    pub fn build(_gate_way: &GatewayConfig) -> Proxy {
        Proxy {}
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
        let c =
            config_store::get().map_err(|_| DakiaError::create_internal().to_pingora_error())?;
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
