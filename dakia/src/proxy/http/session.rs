use std::{collections::HashMap, mem::take, sync::Arc};

use http::{uri::PathAndQuery, StatusCode};
use pingora_http::{RequestHeader as PRequestHeader, ResponseHeader as PResponseHeader};
use pingora_proxy::Session as PSession;

use crate::{
    error::DakiaResult,
    gateway::interceptor::{Hook, HookMask, Interceptor, Phase, PhaseResult},
};

use super::DakiaHttpGatewayCtx;

pub struct Session<'a> {
    psession: &'a mut PSession,
    upstream_request: Option<&'a mut PRequestHeader>,
    downstream_respons: Option<&'a mut PResponseHeader>,
    phase: Phase,
    ds_hbuf: HashMap<String, Vec<u8>>,
    ds_status_code: StatusCode,
    ctx: &'a DakiaHttpGatewayCtx,
}

impl<'a> Session<'a> {
    pub fn build(phase: Phase, psession: &'a mut PSession, ctx: &'a DakiaHttpGatewayCtx) -> Self {
        Session {
            phase,
            psession,
            upstream_request: None,
            downstream_respons: None,
            ds_hbuf: HashMap::new(),
            ds_status_code: StatusCode::OK,
            ctx,
        }
    }

    pub fn upstream_request(&mut self, upstream_request: &'a mut PRequestHeader) {
        self.upstream_request = Some(upstream_request);
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
    fn set_us_header(&self, header_name: String, header_value: &[u8]) {
        // TODO: upstream header can be only added in PreUpstreamRequest phase
        todo!()
    }

    pub fn set_ds_header(&mut self, header_name: String, header_value: Vec<u8>) {
        self.ds_hbuf.insert(header_name, header_value);
    }

    pub async fn flush_ds_header(&mut self) -> DakiaResult<()> {
        let cur_hook_mask = Hook::PreDownstreamResponseHeaderFlush.mask();
        // TODO: allow to configure keepalive once bug is fixed in pingora itself
        // https://github.com/cloudflare/pingora/issues/540
        self.psession.set_keepalive(None);

        self.execute_hooked_interceptors(cur_hook_mask)?;

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
}

impl<'a> Session<'a> {
    pub fn set_ds_res_status(&mut self, status_code: StatusCode) -> DakiaResult<()> {
        self.ds_status_code = status_code;
        Ok(())
    }
}

impl<'a> Session<'a> {
    fn execute_hooked_interceptors(&mut self, cur_hook_mask: HookMask) -> PhaseResult {
        let interceptors = self.ctx.gateway_state.interceptors();

        for interceptor in interceptors {
            let is_phase_enabled: bool = match interceptor.phase_mask() {
                // TODO: create method in Phase for checking if flag is on or not
                Some(phase_mask) => (self.phase.mask() & phase_mask) != 0,
                None => true,
            };

            if !is_phase_enabled || !interceptor.filter(self)? {
                return Ok(true);
            }

            let is_hook_enabled = match interceptor.hook_mask() {
                // TODO: create method in Hook for checking if flag is on or not
                Some(interceptor_hook_mask) => (interceptor_hook_mask & cur_hook_mask) != 0,
                None => false,
            };

            if !is_hook_enabled {
                return Ok(true);
            }

            match self.phase {
                Phase::RequestFilter => interceptor.request_filter(self),
                Phase::UpstreamProxyFilter => interceptor.upstream_proxy_filter(self),
                Phase::PreUpstreamRequest => interceptor.pre_upstream_request(self),
                Phase::PostUpstreamResponse => interceptor.post_upstream_response(self),
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
