use serde;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum PatternType {
    Text,
    Wildcard,
    Regex,
}
