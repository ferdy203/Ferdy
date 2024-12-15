mod config;
mod libs;

use crate::config::dakia_config::DakiaConfig;
use async_trait::async_trait;
use clap::Parser;
use libs::utils::get_dakia_ascii_art;
use pingora::{
    http::RequestHeader,
    prelude::HttpPeer,
    proxy::{http_proxy_service, ProxyHttp, Session},
    server::Server,
    Error,
};

use config::args::{self, DakiaArgs};

struct DakiaCtx {}
impl DakiaCtx {
    fn new() -> DakiaCtx {
        DakiaCtx {}
    }
}

struct DakiaProxy {
    dakia_config: DakiaConfig,
}

#[async_trait]
impl ProxyHttp for DakiaProxy {
    /// For this small example, we don't need context storage
    type CTX = DakiaCtx;

    fn new_ctx(&self) -> Self::CTX {
        DakiaCtx::new()
    }

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        _ctx: &mut DakiaCtx,
    ) -> Result<Box<HttpPeer>, Box<Error>> {
        let peer = Box::new(HttpPeer::new(
            "1.1.1.1:443",
            true,
            "one.one.one.one".to_string(),
        ));

        Ok(peer)
    }

    async fn upstream_request_filter(
        &self,
        _session: &mut Session,
        upstream_request: &mut RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<(), Box<Error>> {
        upstream_request
            .insert_header("Host", "one.one.one.one")
            .unwrap();
        Ok(())
    }
}

impl DakiaProxy {
    fn new(args: &DakiaArgs) -> DakiaProxy {
        DakiaProxy {
            dakia_config: DakiaConfig::build(args),
        }
    }
}

fn main() {
    println!("{}", get_dakia_ascii_art());

    let dakia_args = args::DakiaArgs::parse();
    let dakia_config = DakiaConfig::build(&dakia_args);

    let mut server = Server::new(Some(dakia_config.to_pingora_opt())).unwrap();
    server.bootstrap();

    let dakia_proxy = DakiaProxy::new(&dakia_args);
    let mut dakia_proxy_service = http_proxy_service(&server.configuration, dakia_proxy);

    if let Some(router_config) = dakia_config.router_config {
        for inet_address in &router_config.listen {
            let host = &inet_address.host;
            let port = inet_address.port;

            let host_port = host.to_string() + ":" + &port.to_string();
            dakia_proxy_service.add_tcp(&host_port);
        }
    } else {
        dakia_proxy_service.add_tcp("0.0.0.0:80");
        dakia_proxy_service.add_tcp("0.0.0.0:443");
    }

    server.add_service(dakia_proxy_service);
    server.run_forever();
}
