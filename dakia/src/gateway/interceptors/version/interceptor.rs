use crate::{
    error::DakiaResult,
    gateway::interceptor::{Hook, HookMask, Interceptor, InterceptorName},
    proxy::http::Session,
};

pub struct VersionInterceptor {}

impl Interceptor for VersionInterceptor {
    fn name(&self) -> InterceptorName {
        InterceptorName::Version
    }

    fn hook(&self) -> Option<HookMask> {
        Some(Hook::PreDownstreamResponseHeaderFlush.mask())
    }

    fn filter(&self, _session: &Session) -> DakiaResult<bool> {
        todo!()
    }
    //
}
