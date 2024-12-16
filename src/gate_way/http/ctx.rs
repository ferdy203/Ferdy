pub struct GatewayCtx {
    // TODO: use Rc or Arc to store upstream details
    pub upstream_address: Option<String>,
}

impl GatewayCtx {
    pub fn new() -> GatewayCtx {
        GatewayCtx {
            upstream_address: None,
        }
    }
}
