use async_trait::async_trait;
use bytes::Bytes;
use http::StatusCode;
use log::debug;

use crate::{
    error::DakiaResult,
    gateway::interceptor::{
        HeaderBuffers, Interceptor, InterceptorName, Phase, PhaseMask, PhaseResult,
    },
    proxy::http::{HeaderBuffer, Session},
};

pub struct UseFileInterceptor {
    root: String,
    ds_res_header_buffer: HeaderBuffer,
}

impl UseFileInterceptor {
    pub fn build(root: String, header_buffers: HeaderBuffers) -> Self {
        UseFileInterceptor {
            root,
            ds_res_header_buffer: header_buffers.0,
        }
    }

    fn set_config_headers(&self, _session: &mut Session) {
        for (hkey, hval) in &self.ds_res_header_buffer {
            _session.set_ds_header(hkey.to_string(), hval.clone());
        }
    }
}

#[async_trait]
impl Interceptor for UseFileInterceptor {
    fn name(&self) -> InterceptorName {
        InterceptorName::UseFile
    }

    fn phase_mask(&self) -> Option<PhaseMask> {
        Some(Phase::UpstreamProxyFilter.mask())
    }

    fn filter(&self, _session: &mut Session) -> DakiaResult<bool> {
        // TODO: implement filter
        Ok(true)
    }

    async fn upstream_proxy_filter(&self, _session: &mut Session) -> PhaseResult {
        let path = _session.ds_req_path();
        let aboslute_path = format!("{}{}", self.root, path);

        self.set_config_headers(_session);

        match tokio::fs::read(aboslute_path.clone()).await {
            Ok(file_content) => {
                _session.set_ds_header(
                    "Content-Length".to_string(),
                    file_content.len().to_string().as_bytes().to_vec(),
                );
                _session.flush_ds_header().await?;

                _session
                    .write_res_body(Some(Bytes::from(file_content)), true)
                    .await?;
            }
            Err(err) => {
                debug!("can not read file {aboslute_path} - {err}");
                _session.set_res_status(StatusCode::NOT_FOUND);
                _session.flush_ds_header().await?;
            }
        };

        Ok(true)
    }
}
