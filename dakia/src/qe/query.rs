use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::error::{DakiaError, Error};

pub type Map = HashMap<String, Value>;
pub type Array = Vec<Value>;
pub type Query = Map;

#[derive(PartialEq)]
pub enum Operator {
    And,     // logical and
    Or,      // logical or
    Eq,      // equal to
    Ne,      // not equal to
    In,      // in array
    Nin,     // not in array
    Exists,  // value exists
    Matches, // value matches specified regex
}

impl TryFrom<&str> for Operator {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "$and" => Ok(Self::And),
            "$or" => Ok(Self::Or),
            "$eq" => Ok(Self::Eq),
            "$ne" => Ok(Self::Ne),
            "$in" => Ok(Self::In),
            "$nin" => Ok(Self::Nin),
            "$exists" => Ok(Self::Exists),
            "$matches" => Ok(Self::Matches),
            _ => return Err(*DakiaError::create_unknown_msg("Invalid operator!")),
        }
    }
}

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
