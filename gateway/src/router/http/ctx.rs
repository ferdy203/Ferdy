use crate::config::UpstreamNodeConfig;

pub struct DakiaHttpGatewayCtx {
    pub upstream_config: Option<UpstreamNodeConfig>,
}

impl DakiaHttpGatewayCtx {
    pub fn new() -> DakiaHttpGatewayCtx {
        DakiaHttpGatewayCtx {
            upstream_config: None,
        }
    }
}
