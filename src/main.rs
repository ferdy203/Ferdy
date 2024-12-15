mod config;
mod libs;

use crate::config::dakia_config::DakiaConfig;
use async_trait::async_trait;
use clap::Parser;
use libs::utils::get_dakia_ascii_art;
use pingora::{
    http::RequestHeader,
    lb::LoadBalancer,
    prelude::{background_service, HttpPeer, RoundRobin, TcpHealthCheck},
    proxy::{http_proxy_service, ProxyHttp, Session},
    server::Server,
    Error,
};
use std::sync::Arc;

use config::args;

pub struct LB(Arc<LoadBalancer<RoundRobin>>);

#[async_trait]
impl ProxyHttp for LB {
    /// For this small example, we don't need context storage
    type CTX = ();

    fn new_ctx(&self) -> Self::CTX {
        ()
    }

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        _ctx: &mut (),
    ) -> Result<Box<HttpPeer>, Box<Error>> {
        let upstream = self
            .0
            .select(b"", 256) // hash doesn't matter for round robin
            .unwrap();

        println!("upstream peer is: {upstream:?}");
        let host = _session.req_header().headers.get("host").unwrap();
        println!("Host: {}", host.to_str().unwrap());
        // Set SNI to one.one.one.one
        // let peer = Box::new(HttpPeer::new(upstream, true, "one.one.one.one".to_string()));
        let peer = Box::new(HttpPeer::new(
            upstream,
            false,
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

fn main() {
    println!("{}", get_dakia_ascii_art());

    let dakia_args = args::DakiaArgs::parse();
    let dakia_config = DakiaConfig::build(dakia_args);

    let mut server = Server::new(Some(dakia_config.to_pingora_opt())).unwrap();
    server.bootstrap();

    let mut upstreams = LoadBalancer::try_from_iter(["127.0.0.1:8080", "127.0.0.1:8081"]).unwrap();
    let hc = TcpHealthCheck::new();
    upstreams.set_health_check(hc);
    upstreams.health_check_frequency = Some(std::time::Duration::from_secs(1));

    let background = background_service("health check", upstreams);
    let upstreams = background.task();

    let mut lb = http_proxy_service(&server.configuration, LB(upstreams));
    lb.add_tcp("0.0.0.0:6188");
    server.add_service(background);
    server.add_service(lb);
    server.run_forever();
}
