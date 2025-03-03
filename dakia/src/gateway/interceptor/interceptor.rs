use crate::{error::DakiaResult, proxy::http::Session};

use super::{HookMask, InterceptorName, PhaseMask};

pub type PhaseResult = DakiaResult<bool>;

pub trait Interceptor: Send + Sync {
    fn name(&self) -> InterceptorName;

    fn hook_mask(&self) -> Option<HookMask> {
        None
    }

    fn phase_mask(&self) -> Option<PhaseMask> {
        None
    }

    // if there is no filter, it'll be considered as match
    fn filter(&self, _session: &mut Session) -> DakiaResult<bool> {
        Ok(true)
    }

    fn request_filter(&self, _session: &mut Session) -> PhaseResult {
        Ok(false)
    }

    fn upstream_proxy_filter(&self, _session: &mut Session) -> PhaseResult {
        Ok(false)
    }

    fn pre_upstream_request(&self, _session: &mut Session) -> PhaseResult {
        Ok(false)
    }

    fn post_upstream_response(&self, _session: &mut Session) -> PhaseResult {
        Ok(false)
    }

    fn pre_downstream_response(&self, _session: &mut Session) -> PhaseResult {
        Ok(false)
    }

    fn pre_downstream_response_hook(&self, _session: &mut Session) -> PhaseResult {
        Ok(false)
    }
}
