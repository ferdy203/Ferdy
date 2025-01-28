// TODO: users can only modify upstream request and downstream response, so called rewrite
// they can not modify downstream request, because it'll be written by client
// they can not modify upstream response, because it'll be written by upstreamm server

use http::HeaderName;
use pingora_http::{RequestHeader as PRequestHeader, ResponseHeader as PResponseHeader};
use pingora_proxy::Session as PSession;

use crate::error::{DakiaError, DakiaResult};

pub enum Phase {
    RequestFilter,
    UpstreamPeerSelection,
    PreUpstreamRequest,
    PreDownstreamResponse,
}

pub struct Session<'a> {
    psession: &'a PSession,
    upstream_request: Option<&'a mut PRequestHeader>,
    upstream_response: Option<&'a mut PResponseHeader>,
    pub ds: DownstreamSession<'a>,
    pub us: UpstreamSession<'a>,
    phase: Phase,
}

pub struct SessionBuilder<'a> {
    phase: Phase,
    psession: &'a PSession,
    upstream_request: Option<&'a mut PRequestHeader>,
}

impl<'a> SessionBuilder<'a> {
    pub fn new(phase: Phase, psession: &'a PSession) -> Self {
        SessionBuilder {
            phase,
            psession: psession,
            upstream_request: None,
        }
    }

    pub fn pupstream_request(mut self, upstream_request: &'a mut PRequestHeader) -> Self {
        self.upstream_request = Some(upstream_request);
        self
    }

    pub fn build(self) -> Session<'a> {
        let ds = DownstreamSession {
            req: DownstreamRequest {
                psession: self.psession,
            },
            res: DownstreamResponse {
                pupstream_response: None,
            },
        };

        let us = UpstreamSession {
            req: UpstreamRequest {
                pupstream_request: self.upstream_request,
            },
            res: UpstreamResponse {
                pupstream_response: None,
            },
        };

        Session {
            psession: self.psession,
            ds,
            us,
            phase: self.phase,
            upstream_request: None,
            upstream_response: None,
        }
    }
}

impl<'a> Session<'a> {
    // TODO: implement required functions
}

pub struct DownstreamSession<'a> {
    pub req: DownstreamRequest<'a>,
    pub res: DownstreamResponse<'a>,
}

pub struct DownstreamRequest<'a> {
    psession: &'a PSession,
}

impl<'a> DownstreamRequest<'a> {
    pub fn method(&self) -> &str {
        self.psession.as_downstream().req_header().method.as_str()
    }

    pub fn header(&self, header_name: &str) -> Option<&[u8]> {
        let header_value = self
            .psession
            .as_downstream()
            .req_header()
            .headers
            .get(header_name)?;
        Some(header_value.as_bytes())
    }

    pub fn path(&self) -> &str {
        self.psession.as_downstream().req_header().uri.path()
    }

    pub fn _body(&self) {
        todo!()
    }

    pub fn _http_version(&self) {
        todo!()
    }
}

// allow to write only response headers, response body
pub struct DownstreamResponse<'a> {
    pupstream_response: Option<&'a mut PResponseHeader>,
}

impl<'a> DownstreamResponse<'a> {
    pub fn set_header(&mut self, header_name: String, header_value: &[u8]) -> DakiaResult<()> {
        match &mut self.pupstream_response {
            Some(response_header) => response_header.append_header(header_name, header_value),
            None => {
                return Err(DakiaError::create_internal_context(
                    "Something went wrong! Upstream response required here!",
                ));
            }
        }?;
        Ok(())
    }
}
pub struct UpstreamSession<'a> {
    pub req: UpstreamRequest<'a>,
    pub res: UpstreamResponse<'a>,
}

pub struct UpstreamRequest<'a> {
    pupstream_request: Option<&'a mut PRequestHeader>,
}

impl<'a> UpstreamRequest<'a> {
    pub fn set_path_and_query(&mut self, path: &str, query: Option<&str>) -> DakiaResult<()> {
        let pnq = match query {
            Some(query) => format!("{}?{}", path, query),
            None => path.to_string(),
        };

        let uri = http::Uri::builder().path_and_query(pnq).build()?;

        let _ = match &mut self.pupstream_request {
            Some(request_header) => request_header.set_uri(uri),
            None => {
                return Err(DakiaError::create_internal_context(
                    "Something went wrong! Upstream request required here!",
                ))
            }
        };

        Ok(())
    }

    pub fn set_header(&mut self, header_name: String, header_value: &[u8]) -> DakiaResult<()> {
        match &mut self.pupstream_request {
            Some(request_header) => request_header.append_header(header_name, header_value),
            None => {
                return Err(DakiaError::create_internal_context(
                    "Something went wrong! Upstream request required here!",
                ));
            }
        }?;
        Ok(())
    }

    pub fn _set_body(&mut self) {
        todo!()
    }
}

pub struct UpstreamResponse<'a> {
    pupstream_response: Option<&'a mut PResponseHeader>,
}

impl<'a> UpstreamResponse<'a> {
    pub fn header(&self, header_name: &str) -> DakiaResult<Option<&[u8]>> {
        match &self.pupstream_response {
            Some(response_header) => {
                let header_value = response_header.headers.get(header_name)?;
                Ok(Some(header_value.as_bytes()))
            }
            None => {
                return Err(DakiaError::create_internal_context(
                    "Something went wrong! Upstream response required here!",
                ))
            }
        }
    }

    pub fn _body(&self) {
        todo!()
    }

    pub fn status_code(&self) -> DakiaResult<http::StatusCode> {
        match &self.pupstream_response {
            Some(response_header) => Ok(response_header.status),
            None => {
                return Err(DakiaError::create_internal_context(
                    "Something went wrong! Upstream response required here!",
                ))
            }
        }
    }
}
