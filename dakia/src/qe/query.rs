use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::error::{DakiaError, Error};

pub type Map = HashMap<String, Value>;
pub type Array = Vec<Value>;
pub type Query = Map;

#[derive(PartialEq)]
pub enum Operator {
    And,          // logical and
    Or,           // logical or
    Eq,           // equal to
    Ne,           // not equal to
    In,           // in array
    Nin,          // not in array
    Contains,     // substring present
    NotContains,  // sub strig not present,
    StartsWith,   // text starts with
    NotStartWith, // text not starts with
    EndsWith,     // text ends with
    NotEndsWith,  // text not ends with
    Exists,       // value exists
    Matches,      // value matches specified regex
}

impl TryFrom<&str> for Operator {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "$and" => Ok(Self::And),
            "$or" => Ok(Self::Or),
            "$eq" => Ok(Self::Eq),
            "$not_eq" => Ok(Self::Ne),
            "$in" => Ok(Self::In),
            "$not_in" => Ok(Self::Nin),
            "$exists" => Ok(Self::Exists),
            "$matches" => Ok(Self::Matches),
            "$contains" => Ok(Self::Contains),
            "$not_contains" => Ok(Self::NotContains),
            "$starts_with" => Ok(Self::StartsWith),
            "$not_starts_with" => Ok(Self::NotStartWith),
            "$ends_with" => Ok(Self::EndsWith),
            "$not_ends_with" => Ok(Self::NotEndsWith),
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
