use crate::config::UpstreamConfig;

pub struct DakiaHttpGatewayCtx {
    pub upstream_config: Option<UpstreamConfig>,
}

impl DakiaHttpGatewayCtx {
    pub fn new() -> DakiaHttpGatewayCtx {
        DakiaHttpGatewayCtx {
            upstream_config: None,
        }
    }
}
