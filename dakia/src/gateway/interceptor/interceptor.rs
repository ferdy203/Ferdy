use crate::{error::DakiaResult, proxy::http::Session, qe::query::Query};

use super::hook::Hook;

pub enum PhaseResultValue {
    Bool(bool),
    Tuple(bool, Query),
}

impl Into<PhaseResultValue> for bool {
    fn into(self) -> PhaseResultValue {
        PhaseResultValue::Bool(self)
    }
}

type PhaseResult = DakiaResult<PhaseResultValue>;

pub trait Interceptor<'a> {
    fn name(&self) -> &'a str;
    fn hook(&self) -> Option<&'a Hook>;

    // if there is no filter, it'll be considered as match
    fn filter(&self, _session: &Session) -> DakiaResult<bool>;

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
}
