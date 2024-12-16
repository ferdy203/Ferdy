pub struct DakiaHttpGatewayCtx {
    // TODO: use Rc or Arc to store upstream details
    pub upstream_address: Option<String>,
}

impl DakiaHttpGatewayCtx {
    pub fn new() -> DakiaHttpGatewayCtx {
        DakiaHttpGatewayCtx {
            upstream_address: None,
        }
    }
}
