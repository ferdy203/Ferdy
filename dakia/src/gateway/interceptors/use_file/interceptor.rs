use crate::{
    error::DakiaResult,
    gateway::interceptor::{
        Hook, HookMask, Interceptor, InterceptorName, Phase, PhaseMask, PhaseResult,
    },
    proxy::http::Session,
};

pub struct UseFileInterceptor {}

impl Interceptor for UseFileInterceptor {
    fn name(&self) -> InterceptorName {
        InterceptorName::UseFile
    }

    fn hook_mask(&self) -> Option<HookMask> {
        Some(Hook::PreDownstreamResponseHeaderFlush.mask())
    }

    fn phase_mask(&self) -> Option<PhaseMask> {
        Some(Phase::UpstreamProxyFilter.mask())
    }

    fn filter(&self, _session: &mut Session) -> DakiaResult<bool> {
        // TODO: implement filter
        Ok(true)
    }

    fn upstream_proxy_filter(&self, _session: &mut Session) -> PhaseResult {
        println!("upstream_proxy_filter() in called");
        // TODO: here
        /*

                 server {
            listen 80;
            server_name example.com;

            location /api/response {
                root /var/www/html;
                add_header Content-Type application/json;
                default_type application/json;
                try_files /response.json =404;
            }
        }


                 */
        Ok(false.into())
    }
}
