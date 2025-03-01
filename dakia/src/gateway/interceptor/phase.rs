use std::fmt;

#[derive(PartialEq, Clone, Debug, Eq)]
pub enum Phase {
    RequestFilter,
    UpstreamProxyFilter,
    PreUpstreamRequest,
    PostUpstreamResponse,
}

impl Phase {
    fn to_number(&self) -> u8 {
        match self {
            Phase::RequestFilter => 1,
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
            Phase::RequestFilter => "request_filter",
            Phase::UpstreamProxyFilter => "upstream_proxy_filter",
            Phase::PreUpstreamRequest => "pre_upstream_request",
            Phase::PostUpstreamResponse => "post_upstream_response",
        };
        write!(f, "{}", phase_str)
    }
}
