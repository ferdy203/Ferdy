mod config;
mod error;
mod gateway;
mod globals;
mod proxy;
mod shared;

// use crate::config::DakiaConfigTemp;
use clap::Parser;
use config::{DakiaArgs, DakiaConfig};
use error::DakiaError;
use globals::CONFIG_STORE;
use pingora::server::Server;
use shared::get_dakia_ascii_art;

use proxy::http::Proxy;
use shared::IntoRef;

fn main() -> Result<(), Box<DakiaError>> {
    let dakia_args = DakiaArgs::parse();

    // process args and exist if required
    process_args(&dakia_args);

    let dakia_config = DakiaConfig::from_args(dakia_args.clone())?;

    // perform init steps
    init(&dakia_config);

    // TODO: handle unwrap here
    // TODO: can we avoid using unsafe here?
    #[allow(static_mut_refs)]
    unsafe {
        CONFIG_STORE.store_config(dakia_config.clone()).unwrap();
    };

    let mut server =
        Server::new_with_opt_and_conf(dakia_config.into_ref(), dakia_config.into_ref());

    server.bootstrap();

    for gateway in dakia_config.gateways {
        gateway::init(&mut server, &gateway);
    }

    server.run_forever();
}

fn init(_dakia_config: &DakiaConfig) {
    env_logger::init();
    println!("{}", get_dakia_ascii_art());

    // if error log file option is available then create one
    // if out log file option is avaibale then cretae one
    // TODO: create folder for extensions, filters, interceptors (if valid dp is available)
}

fn process_args(args: &DakiaArgs) -> () {
    if args.version {
        let package_version = env!("CARGO_PKG_VERSION");
        println!("Dakia {}", package_version);
        shared::exit();
    }
}
