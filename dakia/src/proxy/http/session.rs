/*
- Downstream request
    - it's read only
    - user's can read downstream request in any phase
- Upstream request
    - it's read & write request
    - user's can read it in and after PreUpstreamRequest phase
    - user's can write it only in PreUpstreamRequest, there should be error for any write attempt after PreUpstreamRequest phase
- Downstream response
    - user's can write it in any phase
- Upstream response is read only
    - it's read only
    - users's can access upstream response in PreDownstreamResponse phase
-
*/
use std::{
    collections::HashMap,
    fmt::{self},
    mem::take,
};

use http::{uri::PathAndQuery, StatusCode};
use pingora_http::{RequestHeader as PRequestHeader, ResponseHeader as PResponseHeader};
use pingora_proxy::Session as PSession;

use crate::error::{DakiaError, DakiaResult, ImmutStr};

#[derive(PartialEq, Clone, Debug, Eq)]
pub enum Phase {
    Filter,
    UpstreamProxyFilter,
    PreUpstreamRequest,
    PostUpstreamResponse,
}

impl Phase {
    fn to_number(&self) -> u8 {
        match self {
            Phase::Filter => 1,
            Phase::UpstreamProxyFilter => 2,
            Phase::PreUpstreamRequest => 3,
            Phase::PostUpstreamResponse => 4,
        }
    }
}

impl Ord for Phase {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_number().cmp(&other.to_number())
    }
}

impl PartialOrd for Phase {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Phase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let phase_str = match self {
            Phase::Filter => "filter",
            Phase::UpstreamProxyFilter => "upstream_proxy_filter",
            Phase::PreUpstreamRequest => "pre_upstream_request",
            Phase::PostUpstreamResponse => "post_upstream_response",
        };
        write!(f, "{}", phase_str)
    }
}

enum Stream {
    Downstream,
    Upstream,
}

#[derive(PartialEq, Debug, Eq)]
enum Channel {
    Request,
    Response,
}

pub struct Session<'a> {
    psession: &'a mut PSession,
    upstream_request: Option<&'a mut PRequestHeader>,
    downstream_respons: Option<&'a mut PResponseHeader>,
    phase: Phase,
    stream: Stream,
    channel: Channel,
    ds_hbuf: HashMap<String, &'a [u8]>,
    ds_status_code: StatusCode,
}

pub struct SessionBuilder<'a> {
    phase: Phase,
    upstream_request: Option<&'a mut PRequestHeader>,
    psession: &'a mut PSession,
}

impl<'a> Session<'a> {
    pub fn build(phase: Phase, psession: &'a mut PSession) -> Self {
        Session {
            phase,
            psession,
            channel: Channel::Request,
            stream: Stream::Downstream,
            upstream_request: None,
            downstream_respons: None,
            ds_hbuf: HashMap::new(),
            ds_status_code: StatusCode::OK,
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

    fn set_ds_header(&mut self, header_name: String, header_value: &'a [u8]) {
        self.ds_hbuf.insert(header_name, &header_value);
    }

    pub async fn flush_ds_res_header(&mut self) -> DakiaResult<()> {
        if let Stream::Upstream = self.stream {
            // No action is needed for upstream headers.
            // Header writing is only enforced for downstream because we allow writing the body
            // only in the downstream stream. To ensure proper sequencing, the header must be
            // written before the body.
            // This method ensures that dakia internal headers are also written along with client's header
            // This method is designed to support body writing for upstream streams in the future.
            return Ok(());
        }

        // TODO: allow to configure keepalive once bug is fixed in pingora itself
        // https://github.com/cloudflare/pingora/issues/540
        self.psession.set_keepalive(None);

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
// TODO: move this assert into shared module
fn assert(cond: bool, msg: String) -> DakiaResult<()> {
    Ok(if !cond {
        return Err(DakiaError::i_explain(ImmutStr::Owned(msg.into_boxed_str())));
    })
}
