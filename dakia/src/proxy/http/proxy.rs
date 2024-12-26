use crate::config::source_config::GatewayConfig;

use super::DakiaHttpGatewayCtx;
use crate::globals::CONFIG_STORE;
use async_trait::async_trait;
use pingora::{
    prelude::HttpPeer,
    proxy::{ProxyHttp, Session},
    Error,
};

pub struct Proxy {}

impl Proxy {
    pub fn build(_gate_way: &GatewayConfig) -> Proxy {
        Proxy {
            // downstream_patterns: DakiaHttpGateway::get_hosts(gate_way),
            // route_pattern_backend_matchers: DakiaHttpGateway::get_path_map(gate_way),
            // backend_map: DakiaHttpGateway::get_backend_map(gate_way),
            // default_backend: DakiaHttpGateway::get_default_backend(gate_way),
        }
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
        // TODO: fix unwrap here
        // TODO: how can we avoid putting unsafe everywhere? Can we make a function unsafe itself?
        let c = unsafe { CONFIG_STORE.get_latest_config().unwrap() };

        _ctx.config = c;
        println!("early_request_filter Called..");
        Ok(())
    }

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>, Box<Error>> {
        println!("{}", _ctx.config.daemon);

        println!("upstream_peer Called..");
        let addr = ("127.0.0.1", 3000);

        let peer = Box::new(HttpPeer::new(addr, false, "one.one.one.one".to_string()));
        Ok(peer)
    }
}
