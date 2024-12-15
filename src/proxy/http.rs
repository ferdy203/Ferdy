use crate::config::{args::DakiaArgs, dakia_config::DakiaConfig};

pub struct DakiaHttpProxy {
    dakia_config: DakiaConfig,
}

impl DakiaHttpProxy {
    pub fn new(args: &DakiaArgs) -> DakiaHttpProxy {
        DakiaHttpProxy {
            dakia_config: DakiaConfig::build(args),
        }
    }
}

pub struct DakiaCtx {}

impl DakiaCtx {
    pub fn new() -> DakiaCtx {
        DakiaCtx {}
    }
}
