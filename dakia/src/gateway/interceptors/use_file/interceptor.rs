use crate::{
    error::DakiaResult,
    gateway::interceptor::{
        Hook, HookMask, Interceptor, InterceptorName, Phase, PhaseMask, PhaseResult,
    },
    proxy::http::{HeaderBuffer, Session},
};

pub struct UseFileInterceptor {
    root: String,
    ds_res_header_buffer: HeaderBuffer,
    us_req_header_buffer: HeaderBuffer,
}

impl UseFileInterceptor {
    pub fn build(root: String, header_buffers: (HeaderBuffer, HeaderBuffer)) -> Self {
        UseFileInterceptor {
            root,
            ds_res_header_buffer: header_buffers.0,
            us_req_header_buffer: header_buffers.1,
        }
    }
}
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
