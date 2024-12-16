mod config;
mod libs;
mod proxy;

use crate::config::dakia_config::DakiaConfig;
use async_trait::async_trait;
use clap::Parser;
use config::args::DakiaArgs;
use libs::utils::{get_dakia_ascii_art, get_or_default};
use pingora::{
    http::RequestHeader,
    prelude::HttpPeer,
    proxy::{http_proxy_service, ProxyHttp, Session},
    server::Server,
    Error,
};

use proxy::http::{DakiaCtx, DakiaHttpProxy};

#[async_trait]
impl ProxyHttp for DakiaHttpProxy {
    type CTX = DakiaCtx;

    fn new_ctx(&self) -> Self::CTX {
        DakiaCtx::new()
    }

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        _ctx: &mut DakiaCtx,
    ) -> Result<Box<HttpPeer>, Box<Error>> {
        let host_header_value = _session.req_header().headers.get("host");

        if host_header_value.is_none() {
            return Err(Error::new(pingora::ErrorType::ConnectNoRoute));
        }

        let host_header_str_result = host_header_value.unwrap().to_str();
        if host_header_str_result.is_err() {
            return Err(Error::new(pingora::ErrorType::ReadError));
        }

        let host_header_str = host_header_str_result.unwrap();
        let path = _session.req_header().uri.path();
        let upstream = self.get_up_stream(host_header_str.to_string(), path.to_string());

        match upstream {
            Some(upstream) => {
                let address = format!(
                    "{}:{}",
                    upstream.inet_address.host, upstream.inet_address.port
                );
                _ctx.upstream_address = Some(address.clone());

                let peer = Box::new(HttpPeer::new(
                    address,
                    upstream.tls,
                    // TODO: avoid clone here
                    get_or_default(upstream.sni.clone(), "".to_string()),
                ));
                Ok(peer)
            }
            None => Err(Error::new(pingora::ErrorType::ConnectNoRoute)),
        }
    }

    async fn upstream_request_filter(
        &self,
        _session: &mut Session,
        upstream_request: &mut RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<(), Box<Error>> {
        let addr = _ctx.upstream_address.as_ref().unwrap();
        upstream_request.insert_header("Host", addr).unwrap();
        Ok(())
    }
}

// TODO: refactor entire code to improve code quality and organization
// TODO: add regex host and path matching along with wild card host and path matching
fn main() {
    println!("{}", get_dakia_ascii_art());

    let dakia_args = DakiaArgs::parse();
    let dakia_config = DakiaConfig::build(&dakia_args);

    let mut server = Server::new(Some(dakia_config.to_pingora_opt())).unwrap();
    server.bootstrap();

    if let Some(router_config) = dakia_config.router_config {
        for gate_way in &router_config.gate_ways {
            let dakia_proxy = DakiaHttpProxy::build(gate_way);
            let mut dakia_proxy_service = http_proxy_service(&server.configuration, dakia_proxy);

            for inet_address in &gate_way.listen {
                let host = &inet_address.host;
                let port = inet_address.port;

                let addr = format!("{}:{}", host, port);
                dakia_proxy_service.add_tcp(&addr);
            }

            server.add_service(dakia_proxy_service);
        }
    }

    server.run_forever();
}
