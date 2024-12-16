use crate::config::router_config::Gateway;

pub struct DakiaHttpProxy {
    dakia_gate_way: Gateway,
}

impl DakiaHttpProxy {
    pub fn new(gate_way: &Gateway) -> DakiaHttpProxy {
        DakiaHttpProxy {
            dakia_gate_way: gate_way.clone(),
        }
    }
}

pub struct DakiaCtx {}

impl DakiaCtx {
    pub fn new() -> DakiaCtx {
        DakiaCtx {}
    }
}
