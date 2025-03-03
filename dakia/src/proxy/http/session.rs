use std::{collections::HashMap, mem::take, sync::Arc};

use http::{uri::PathAndQuery, StatusCode};
use pingora_http::{RequestHeader as PRequestHeader, ResponseHeader as PResponseHeader};
use pingora_proxy::Session as PSession;

use crate::{
    error::{DakiaError, DakiaResult},
    gateway::interceptor::{Hook, Interceptor, Phase, PhaseResult},
};

use super::DakiaHttpGatewayCtx;

pub struct Session<'a> {
    psession: &'a mut PSession,
    upstream_request: Option<&'a mut PRequestHeader>,
    upstream_response: Option<&'a mut PResponseHeader>,
    phase: Phase,
    ds_hbuf: HeaderBuffer,
    us_hbuf: HeaderBuffer,
    ds_status_code: StatusCode,
    ctx: &'a DakiaHttpGatewayCtx,
}

impl<'a> Session<'a> {
    pub fn build(phase: Phase, psession: &'a mut PSession, ctx: &'a DakiaHttpGatewayCtx) -> Self {
        Session {
            phase,
            psession,
            upstream_request: None,
            upstream_response: None,
            ds_hbuf: HeaderBuffer::new(),
            us_hbuf: HeaderBuffer::new(),
            ds_status_code: StatusCode::OK,
            ctx,
        }
    }

    pub fn upstream_request(&mut self, upstream_request: &'a mut PRequestHeader) {
        self.upstream_request = Some(upstream_request);
    }

    pub fn upstream_response(&mut self, upstream_response: &'a mut PResponseHeader) {
        self.upstream_response = Some(upstream_response);
    }
}

impl<'a> Session<'a> {
    pub fn ds_req_method(&self) -> DakiaResult<&str> {
        Ok(self.psession.as_downstream().req_header().method.as_str())
    }

    pub fn us_req_method(&self) -> DakiaResult<&str> {
        Ok(self.upstream_request.as_ref().unwrap().method.as_str())
    }
}

impl<'a> Session<'a> {
    pub fn ds_req_path(&self) -> &str {
        self.psession.as_downstream().req_header().uri.path()
    }
}

impl<'a> Session<'a> {
    pub fn ds_req_query(&self) -> DakiaResult<Option<&str>> {
        Ok(self.psession.as_downstream().req_header().uri.query())
    }

    pub fn us_req_query(&self) -> DakiaResult<Option<&str>> {
        Ok(self.upstream_request.as_ref().unwrap().uri.query())
    }

    pub fn ds_req_path_and_query(&self) -> Option<&PathAndQuery> {
        self.psession
            .as_downstream()
            .req_header()
            .uri
            .path_and_query()
    }

    pub fn us_req_path_and_query(&self) -> Option<&PathAndQuery> {
        self.upstream_request.as_ref().unwrap().uri.path_and_query()
    }
}

impl<'a> Session<'a> {
    pub fn us_req_header(&self, header_name: &str) -> DakiaResult<Option<&[u8]>> {
        let header_value = self
            .upstream_request
            .as_ref()
            .unwrap()
            .headers
            .get(header_name);

        match header_value {
            Some(value) => Ok(Some(value.as_bytes())),
            None => Ok(None),
        }
    }

    pub fn ds_req_header(&self, header_name: &str) -> DakiaResult<Option<&[u8]>> {
        let header_value = self
            .psession
            .as_downstream()
            .req_header()
            .headers
            .get(header_name);

        match header_value {
            Some(value) => Ok(Some(value.as_bytes())),
            None => Ok(None),
        }
    }
}

impl<'a> Session<'a> {
    pub fn set_us_header(&mut self, header_name: String, header_value: Vec<u8>) {
        self.us_hbuf.insert(header_name, header_value);
    }

    pub fn set_ds_header(&mut self, header_name: String, header_value: Vec<u8>) {
        self.ds_hbuf.insert(header_name, header_value);
    }

    async fn flush_header_in_ds(&mut self) -> DakiaResult<()> {
        let mut header = PResponseHeader::build(self.ds_status_code, None).unwrap();

        let headers = take(&mut self.ds_hbuf);
        for (header_name, header_value) in headers.into_iter() {
            header.insert_header(header_name, header_value)?;
        }

        self.psession
            .write_response_header(Box::new(header), false)
            .await?;

        Ok(())
    }

    async fn flush_header_in_us_res(&mut self) -> DakiaResult<()> {
        let upstream_response = self.upstream_response.as_mut().expect(
            format!(
                "upstream_response must be available in phase {}",
                Phase::PostUpstreamResponse
            )
            .as_str(),
        );

        let headers = take(&mut self.ds_hbuf);
        println!("Headers: {}", headers.len());
        for (header_name, header_value) in headers.into_iter() {
            upstream_response.insert_header(header_name, header_value)?;
        }

        Ok(())
    }

    pub async fn flush_ds_header(&mut self) -> DakiaResult<()> {
        let cur_hook = Hook::PreDownstreamResponseHeaderFlush;
        // TODO: allow to configure keepalive once bug is fixed in pingora itself
        // https://github.com/cloudflare/pingora/issues/540
        self.psession.set_keepalive(None);

        self.execute_hooked_interceptors(cur_hook)?;

        match self.phase {
            Phase::RequestFilter | Phase::UpstreamProxyFilter | Phase::PreDownstreamResponse => {
                self.flush_header_in_ds().await
            }
            Phase::PreUpstreamRequest => Err(DakiaError::i_explain(format!(
                "can not write downstream headers in {} phase",
                Phase::PreUpstreamRequest
            ))),
            Phase::PostUpstreamResponse => self.flush_header_in_us_res().await,
        }
    }
}

impl<'a> Session<'a> {
    pub fn set_res_status(&mut self, status_code: StatusCode) {
        self.ds_status_code = status_code;
    }
}

impl<'a> Session<'a> {
    fn execute_hooked_interceptors(&mut self, cur_hook: Hook) -> PhaseResult {
        let interceptors = self.ctx.gateway_state.interceptors();

        for interceptor in interceptors {
            match cur_hook {
                Hook::PreDownstreamResponseHeaderFlush => {
                    interceptor.pre_downstream_response_hook(self)
                }
            }?;
        }
        Ok(false)
    }

    fn execute_interceptor(&mut self, interceptor: &Arc<dyn Interceptor>) -> PhaseResult {
        match self.phase {
            Phase::RequestFilter => interceptor.request_filter(self),
            Phase::UpstreamProxyFilter => interceptor.upstream_proxy_filter(self),
            Phase::PreUpstreamRequest => interceptor.pre_upstream_request(self),
            Phase::PostUpstreamResponse => interceptor.post_upstream_response(self),
            Phase::PreDownstreamResponse => interceptor.pre_downstream_response(self),
        }
    }

    pub fn execute_interceptors(&mut self) -> PhaseResult {
        let interceptors = self.ctx.gateway_state.interceptors();

        for interceptor in interceptors {
            if interceptor.filter(self)? && interceptor.hook_mask().is_none() {
                let phase_result = self.execute_interceptor(interceptor)?;
                if phase_result {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }
}

pub type HeaderBuffer = HashMap<String, Vec<u8>>;
