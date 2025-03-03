use crate::{
    error::DakiaResult,
    proxy::http::{HeaderBuffer, Session},
};

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

    fn ds_res_header_buffer(&self) -> Option<&HeaderBuffer> {
        None
    }

    fn us_req_header_buffer(&self) -> Option<&HeaderBuffer> {
        None
    }

    fn init(&self, _session: &mut Session) -> DakiaResult<()> {
        match self.ds_res_header_buffer() {
            Some(header_buffer) => {
                for (hkey, hval) in header_buffer {
                    _session.set_ds_header(hkey.clone(), hval.clone());
                }
            }
            None => (),
        };

        match self.us_req_header_buffer() {
            Some(header_buffer) => {
                for (hkey, hval) in header_buffer {
                    _session.set_us_header(hkey.clone(), hval.clone());
                }
            }
            None => (),
        };
        Ok(())
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
