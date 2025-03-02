use crate::{error::DakiaResult, proxy::http::Session, qe::query::Query};

use super::{HookMask, InterceptorName};

pub enum PhaseResultValue {
    Bool(bool),
    Tuple(bool, Query),
}

impl Into<PhaseResultValue> for bool {
    fn into(self) -> PhaseResultValue {
        PhaseResultValue::Bool(self)
    }
}

pub type PhaseResult = DakiaResult<PhaseResultValue>;

pub trait Interceptor: Send + Sync {
    fn name(&self) -> InterceptorName;
    fn hook(&self) -> Option<HookMask>;

    // if there is no filter, it'll be considered as match
    fn filter(&self, _session: &Session) -> DakiaResult<bool>;

    fn request_filter(&mut self, _session: &Session) -> PhaseResult {
        Ok(false.into())
    }

    fn upstream_proxy_filter(&mut self, _session: &Session) -> PhaseResult {
        Ok(false.into())
    }

    fn pre_upstream_request(&mut self, _session: &Session) -> PhaseResult {
        Ok(false.into())
    }

    fn post_upstream_response(&mut self, _session: &Session) -> PhaseResult {
        Ok(false.into())
    }
}
