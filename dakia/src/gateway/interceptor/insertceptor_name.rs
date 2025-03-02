#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InterceptorName {
    ServerVersion,
}

impl InterceptorName {
    pub fn as_str(&self) -> &'static str {
        match self {
            InterceptorName::ServerVersion => "server_version",
        }
    }
}
