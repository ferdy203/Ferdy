use crate::{
    error::DakiaResult,
    gateway::interceptor::{Hook, HookMask, Interceptor, InterceptorName, PhaseMask, PhaseResult},
    proxy::http::Session,
};

pub struct VersionInterceptor {}

impl Interceptor for VersionInterceptor {
    fn name(&self) -> InterceptorName {
        InterceptorName::Version
    }

    fn hook_mask(&self) -> Option<HookMask> {
        Some(Hook::PreDownstreamResponseHeaderFlush.mask())
    }

    fn filter(&self, _session: &Session) -> DakiaResult<bool> {
        Ok(true)
    }

    fn request_filter(&self, _session: &Session) -> PhaseResult {
        Ok(false.into())
    }

    fn upstream_proxy_filter(&self, _session: &Session) -> PhaseResult {
        Ok(false.into())
    }

    fn pre_upstream_request(&self, _session: &Session) -> PhaseResult {
        Ok(false.into())
    }

    fn post_upstream_response(&self, _session: &Session) -> PhaseResult {
        Ok(false.into())
    }

    fn phase_mask(&self) -> Option<PhaseMask> {
        None
    }
}
