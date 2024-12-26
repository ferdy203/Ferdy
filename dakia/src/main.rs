mod config;
mod globals;
mod libs;
mod proxy;
mod shared;
// use crate::config::DakiaConfigTemp;
use clap::Parser;
use config::DakiaArgs;
use libs::utils::get_dakia_ascii_art;
use pingora::{proxy::http_proxy_service, server::Server};

use proxy::http::Proxy;

// TODO: refactor entire code to improve code quality and organization
// TODO: add regex host and path matching along with wild card host and path matching
fn main() {
    env_logger::init();
    println!("{}", get_dakia_ascii_art());

    // let dakia_args = DakiaArgs::parse();
    // let dakia_config = DakiaConfigTemp::build(&dakia_args);

    // let mut server = Server::new_with_opt_and_conf(
    //     dakia_config.to_pingora_opt(),
    //     dakia_config.to_pingora_server_config(),
    // );

    // server.bootstrap();

    // if let Some(router) = dakia_config.router {
    //     for gate_way in &router.gateways {
    //         let dakia_proxy = Proxy::build(gate_way);
    //         let mut dakia_proxy_service = http_proxy_service(&server.configuration, dakia_proxy);

    //         for inet_address in &gate_way.bind_addresses {
    //             let host = &inet_address.host;
    //             let port = inet_address.port;

    //             let addr = format!("{}:{}", host, port);
    //             dakia_proxy_service.add_tcp(&addr);
    //         }

    //         server.add_service(dakia_proxy_service);
    //     }
    // }

    // server.run_forever();
}
