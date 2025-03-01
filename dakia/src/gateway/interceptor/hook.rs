use std::fmt;

#[derive(PartialEq, Clone, Debug, Eq)]
pub enum Hook {
    PreDownstreamRequestHeaderFlush,
}

impl fmt::Display for Hook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let phase_str = match self {
            Hook::PreDownstreamRequestHeaderFlush => "pre_downstream_request_header_flush",
        };
        write!(f, "{}", phase_str)
    }
}
