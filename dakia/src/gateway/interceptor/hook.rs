use std::fmt;

pub type HookMask = u8;

#[derive(PartialEq, Clone, Debug, Eq)]
pub enum Hook {
    PreDownstreamResponseHeaderFlush = 0x01,
}

impl fmt::Display for Hook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let phase_str = match self {
            Hook::PreDownstreamResponseHeaderFlush => "pre_downstream_request_header_flush",
        };
        write!(f, "{}", phase_str)
    }
}

impl Hook {
    pub fn eq(&self, phase: Hook) -> bool {
        ((self.clone()) as HookMask & phase as HookMask) != 0
    }

    pub fn mask(&self) -> HookMask {
        self.clone() as HookMask
    }
}
