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
use shared::{get_ascii_version, get_dakia_ascii_art};

use proxy::http::Proxy;
use shared::IntoRef;

fn main() -> Result<(), Box<DakiaError>> {
    println!("{}", get_dakia_ascii_art());
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

    // if error log file option is available then create one
    // if out log file option is avaibale then cretae one
    // TODO: add support for access log like nginx
    // TODO: create folder for extensions, filters, interceptors (if valid dp is available)
}

fn process_args(_args: &DakiaArgs) -> () {
    if _args.version {
        // version will be printed along with dakia art in the very beginning
        shared::exit();
    }

    if _args.reload {
        // TODO: add reload support
        shared::exit();
    }

    if _args.debug {
        // TODO: change log level to debug
        shared::exit();
    }

    if _args.test {
        // TODO: validate config
        shared::exit();
    }
}
