use std::sync::OnceLock;

use crate::{
    gateway::interceptor::{Hook, HookMask, Interceptor, InterceptorName, PhaseResult},
    proxy::http::Session,
    shared::common::get_dakia_version,
};

const SERVER_HEADER_NAME: &str = "Server";
static SERVER_HEADER_BYTES: OnceLock<Vec<u8>> = OnceLock::new();

pub struct ServerVersionInterceptor {}

impl ServerVersionInterceptor {
    fn insert_header(&self, _session: &mut Session) -> PhaseResult {
        let header_value =
            SERVER_HEADER_BYTES.get_or_init(|| get_dakia_version().as_bytes().to_vec());

        _session.set_ds_header(SERVER_HEADER_NAME.to_owned(), header_value.clone());
        Ok(false)
    }
}

impl Interceptor for ServerVersionInterceptor {
    fn name(&self) -> InterceptorName {
        InterceptorName::ServerVersion
    }

    fn hook_mask(&self) -> Option<HookMask> {
        Some(Hook::PreDownstreamResponseHeaderFlush.mask())
    }

    fn pre_downstream_response_hook(&self, _session: &mut Session) -> PhaseResult {
        self.insert_header(_session)
    }
}
