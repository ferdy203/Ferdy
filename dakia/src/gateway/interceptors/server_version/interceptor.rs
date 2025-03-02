use crate::{
    error::DakiaResult,
    gateway::interceptor::{Hook, HookMask, Interceptor, InterceptorName, PhaseMask, PhaseResult},
    proxy::http::Session,
    shared::common::get_dakia_version,
};

pub struct ServerVersionInterceptor {}

impl ServerVersionInterceptor {
    fn insert_header(&self, _session: &mut Session) -> PhaseResult {
        let server_header_value = format!("Dakia/{}", get_dakia_version()); // .as_bytes();
        _session.set_ds_header(
            "Server".to_string(),
            server_header_value.as_bytes().to_vec(),
        );
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

    fn phase_mask(&self) -> Option<PhaseMask> {
        None
    }

    fn filter(&self, _session: &mut Session) -> DakiaResult<bool> {
        Ok(true)
    }

    fn request_filter(&self, _session: &mut Session) -> PhaseResult {
        self.insert_header(_session)
    }

    fn upstream_proxy_filter(&self, _session: &mut Session) -> PhaseResult {
        Ok(false.into())
    }

    fn pre_upstream_request(&self, _session: &mut Session) -> PhaseResult {
        Ok(false.into())
    }

    fn post_upstream_response(&self, _session: &mut Session) -> PhaseResult {
        Ok(false.into())
    }

    fn pre_downstream_response(&self, _session: &mut Session) -> PhaseResult {
        self.insert_header(_session)
    }
}
