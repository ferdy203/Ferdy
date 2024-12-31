use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub type Map = HashMap<String, Value>;
pub type Array = Vec<Value>;
pub type Query = HashMap<String, Value>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Scaler(Scaler),
    Composite(Composite),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Scaler {
    String(String),
    I32(i32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Composite {
    Map(Map),
    Array(Array),
}
