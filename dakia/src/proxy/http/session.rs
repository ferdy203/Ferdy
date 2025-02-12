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
use std::fmt::{self};

use http::{uri::PathAndQuery, StatusCode};
use pingora_http::{RequestHeader as PRequestHeader, ResponseHeader as PResponseHeader};
use pingora_proxy::Session as PSession;

use crate::error::{DakiaError, DakiaResult, ImmutStr};

#[derive(PartialEq, Clone, Debug, Eq)]
pub enum Phase {
    RequestFilter,
    UpstreamPeerSelection,
    PreUpstreamRequest,
    PreDownstreamResponse,
}

impl Phase {
    fn to_number(&self) -> u8 {
        match self {
            Phase::RequestFilter => 1,
            Phase::UpstreamPeerSelection => 2,
            Phase::PreUpstreamRequest => 3,
            Phase::PreDownstreamResponse => 4,
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
            Phase::RequestFilter => "Request Filter",
            Phase::UpstreamPeerSelection => "Upstream Peer Selection",
            Phase::PreUpstreamRequest => "Pre-Upstream Request",
            Phase::PreDownstreamResponse => "Pre-Downstream Response",
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
        }
    }

    pub fn upstream_request(&mut self, upstream_request: &'a mut PRequestHeader) {
        self.upstream_request = Some(upstream_request);
    }
}

impl<'a> Session<'a> {
    pub fn ds(&mut self) -> &'a mut Session {
        self.stream = Stream::Downstream;
        self
    }

    pub fn us(&mut self) -> DakiaResult<&'a mut Session> {
        assert(
            self.phase >= Phase::PreUpstreamRequest && self.upstream_request.is_some(),
            format!(
                "Upstream is not ready! It can be accessed only in and after {} phase",
                Phase::PreUpstreamRequest
            ),
        )?;

        self.stream = Stream::Upstream;
        Ok(self)
    }
}

// http setter
impl<'a> Session<'a> {
    pub fn req(&mut self) -> DakiaResult<&'a mut Session> {
        self.channel = Channel::Request;
        Ok(self)
    }

    pub fn res(&mut self) -> DakiaResult<&'a mut Session> {
        // upstream response is made available in PreDownstreamResponse phase
        if let Stream::Upstream = self.stream {
            assert(
                self.phase >= Phase::PreDownstreamResponse,
                format!(
                    "Upstream response is not ready! It can be accessed only in and after {} phase",
                    Phase::PreDownstreamResponse
                ),
            )?;
        }

        self.channel = Channel::Response;
        Ok(self)
    }
}

impl<'a> Session<'a> {
    fn ds_method(&self) -> DakiaResult<&str> {
        Ok(self.psession.as_downstream().req_header().method.as_str())
    }

    fn us_method(&self) -> DakiaResult<&str> {
        Ok(self.upstream_request.as_ref().unwrap().method.as_str())
    }

    pub fn method(&self) -> DakiaResult<&str> {
        match self.stream {
            Stream::Downstream => self.ds_method(),
            Stream::Upstream => self.us_method(),
        }
    }
}

impl<'a> Session<'a> {
    pub fn path(&self) -> &str {
        self.psession.as_downstream().req_header().uri.path()
    }
}

impl<'a> Session<'a> {
    pub fn query(&self) -> DakiaResult<Option<&str>> {
        match self.stream {
            Stream::Downstream => Ok(self.psession.as_downstream().req_header().uri.query()),
            Stream::Upstream => Ok(self.upstream_request.as_ref().unwrap().uri.query()),
        }
    }

    pub fn path_and_query(&self) -> Option<&PathAndQuery> {
        match self.stream {
            Stream::Downstream => self
                .psession
                .as_downstream()
                .req_header()
                .uri
                .path_and_query(),
            Stream::Upstream => self.upstream_request.as_ref().unwrap().uri.path_and_query(),
        }
    }
}

impl<'a> Session<'a> {
    fn us_header(&self, header_name: &str) -> DakiaResult<Option<&[u8]>> {
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

    fn ds_header(&self, header_name: &str) -> DakiaResult<Option<&[u8]>> {
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

    pub fn header(&self, header_name: &str) -> DakiaResult<Option<&[u8]>> {
        match &self.stream {
            Stream::Upstream => self.us_header(header_name),
            Stream::Downstream => self.ds_header(header_name),
        }
    }
}

impl<'a> Session<'a> {
    fn set_us_header(&self, header_name: String, header_value: &[u8]) -> DakiaResult<()> {
        // TODO: upstream header can be only added in PreUpstreamRequest phase
        todo!()
    }

    fn set_ds_header(&self, header_name: String, header_value: &[u8]) -> DakiaResult<()> {
        // TODO: downstream header can be written in Any phase
        todo!()
    }

    pub fn set_header(&self, header_name: String, header_value: &[u8]) -> DakiaResult<()> {
        match &self.stream {
            Stream::Upstream => self.set_us_header(header_name, header_value),
            Stream::Downstream => self.set_ds_header(header_name, header_value),
        }
    }
}

// TODO: move this assert into shared module
fn assert(cond: bool, msg: String) -> DakiaResult<()> {
    Ok(if !cond {
        return Err(DakiaError::i_explain(ImmutStr::Owned(msg.into_boxed_str())));
    })
}
