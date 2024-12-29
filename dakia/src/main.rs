mod config;
mod error;
mod gateway;
mod globals;
mod proxy;
mod shared;

use clap::Parser;
use config::{DakiaArgs, DakiaConfig};
use error::DakiaError;
use globals::config_store;
use pingora::server::Server;
use shared::get_dakia_ascii_art;

use proxy::http::Proxy;
use shared::IntoRef;

fn main() -> Result<(), Box<DakiaError>> {
    println!("{}", get_dakia_ascii_art());
    let dakia_args = DakiaArgs::parse();

    // process args and exist if required
    process_args(&dakia_args)?;

    let dakia_config = DakiaConfig::from_args(dakia_args.clone())?;

    // perform init steps
    init(&dakia_config);

    let mut server =
        Server::new_with_opt_and_conf(dakia_config.into_ref(), dakia_config.into_ref());

    server.bootstrap();

    for gateway in &dakia_config.gateways {
        gateway::init(&mut server, gateway);
    }

    server.run_forever();
}

fn init(_dakia_config: &DakiaConfig) {
    env_logger::init();

    let dc = _dakia_config.clone();

    // pingora uses seprate runtime per config which we don't have access to
    // shutdown_background this runtime, because we no longer need this
    // new runtime is required because we can not access pingora runtime and asyn function needs runtime
    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        // if there is any error, just panic
        .unwrap();

    let h = rt.spawn(async move {
        let _ = config_store::store(dc).await;
    });
    rt.block_on(h).unwrap();
    rt.shutdown_background();
}

fn process_args(_args: &DakiaArgs) -> Result<(), Box<DakiaError>> {
    if _args.version {
        // version will be printed along with dakia art in the very beginning, so just exist from here
        shared::exit();
    }

    if _args.reload {
        // https://www.notion.so/ats1999/Config-reload-16a598d18bbd8090af9ac6f5a902c7b1?pvs=4
        shared::exit();
    }

    if _args.debug {
        // https://www.notion.so/ats1999/Change-Log-level-at-run-time-16a598d18bbd80619c34c90f8952060b?pvs=4
        shared::exit();
    }

    if _args.test {
        // https://www.notion.so/ats1999/Config-Validator-16a598d18bbd80a080f1ef08090f5969?pvs=4
        shared::exit();
    }

    Ok(())
}
