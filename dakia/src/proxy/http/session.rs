// TODO: users can only modify upstream request and downstream response, so called rewrite
// they can not modify downstream request, because it'll be written by client
// they can not modify upstream response, because it'll be written by upstreamm server

use pingora_proxy::Session as PSession;

pub struct Session<'a> {
    psession: &'a PSession,
    pub ds: DownstreamSession<'a>,
    pub us: UpstreamSession,
}

impl<'a> Session<'a> {
    pub fn build(psession: &'a PSession) -> Self {
        let ds = DownstreamSession {
            req: DownstreamRequest { psession },
            res: DownstreamResponse {},
        };

        let us = UpstreamSession {
            req: UpstreamRequest {},
            res: UpstreamResponse {},
        };

        Self { psession, ds, us }
    }
}

pub enum StreamType {
    Ds,
    Us,
}

pub struct StreamSession {
    // session: &'a Session<'a>,
    stream_type: StreamType,
}

impl<'a> StreamSession {
    pub fn req(&self) -> HttpRequest {
        HttpRequest { ss: self }
    }

    pub fn res(&self) -> HttpResponse {
        todo!()
    }
    pub fn session(&self) -> &Session {
        // self.session
        todo!()
    }
}

pub struct HttpRequest<'a> {
    ss: &'a StreamSession,
}

impl<'a> HttpRequest<'a> {
    pub fn method(&self) -> &'a str {
        let ps = self.ss.session().psession;
        match self.ss.stream_type {
            StreamType::Ds => ps.as_downstream().req_header().method.as_str(),
            StreamType::Us => todo!(),
        }
    }
}

pub struct HttpResponse<'a> {
    ss: &'a StreamSession,
}

impl<'a> HttpResponse<'a> {}

pub struct DownstreamSession<'a> {
    pub req: DownstreamRequest<'a>,
    pub res: DownstreamResponse,
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
}

pub struct DownstreamResponse {} // read & write

pub struct UpstreamSession {
    pub req: UpstreamRequest,
    pub res: UpstreamResponse,
}
pub struct UpstreamRequest {} // read & write
pub struct UpstreamResponse {} // read only
