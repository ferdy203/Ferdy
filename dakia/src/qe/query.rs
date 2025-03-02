use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::error::{DakiaError, Error};

pub type Map = HashMap<String, Value>;
pub type Array = Vec<Value>;
pub type Query = Map;

#[derive(PartialEq, Debug)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Value {
    Scaler(Scaler),
    Composite(Composite),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Scaler {
    String(String),
    I32(i32),
    Bool(bool),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Composite {
    Map(Map),
    Vector(Array),
}

// fields of enum SupplierValue should be equivalent to Scaler enum fields of Query
#[derive(Debug)]
pub enum SupplierValue<'a> {
    I32(i32),
    // TODO: change Str to byte to support non UTF-8 characters
    Str(&'a str),
    None,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operator_try_from() {
        assert_eq!(Operator::try_from("$and").unwrap(), Operator::And);
        assert_eq!(Operator::try_from("$or").unwrap(), Operator::Or);
        assert_eq!(Operator::try_from("$eq").unwrap(), Operator::Eq);
        assert_eq!(Operator::try_from("$not_eq").unwrap(), Operator::Ne);
        assert_eq!(Operator::try_from("$in").unwrap(), Operator::In);
        assert_eq!(Operator::try_from("$not_in").unwrap(), Operator::Nin);
        assert_eq!(Operator::try_from("$exists").unwrap(), Operator::Exists);
        assert_eq!(Operator::try_from("$matches").unwrap(), Operator::Matches);
        assert_eq!(Operator::try_from("$contains").unwrap(), Operator::Contains);
        assert_eq!(
            Operator::try_from("$not_contains").unwrap(),
            Operator::NotContains
        );
        assert_eq!(
            Operator::try_from("$starts_with").unwrap(),
            Operator::StartsWith
        );
        assert_eq!(
            Operator::try_from("$not_starts_with").unwrap(),
            Operator::NotStartWith
        );
        assert_eq!(
            Operator::try_from("$ends_with").unwrap(),
            Operator::EndsWith
        );
        assert_eq!(
            Operator::try_from("$not_ends_with").unwrap(),
            Operator::NotEndsWith
        );

        assert!(Operator::try_from("$invalid").is_err());
    }

    #[test]
    fn test_value_serialization() {
        let string_value = Value::Scaler(Scaler::String("hello".to_string()));
        let yaml = serde_yaml::to_string(&string_value).unwrap();
        assert_eq!(yaml.trim(), "hello");
    }

    #[test]
    fn test_value_deserialization() {
        let yaml = "hello";
        let value: Value = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(value, Value::Scaler(Scaler::String("hello".to_string())));
    }

    #[test]
    fn test_composite_map() {
        let mut map = Map::new();
        map.insert("key".to_string(), Value::Scaler(Scaler::I32(42)));
        let composite = Composite::Map(map);
        let yaml = serde_yaml::to_string(&composite).unwrap();
        assert!(yaml.contains("42"));
    }

    #[test]
    fn test_composite_vector() {
        let array = Array::from([
            Value::Scaler(Scaler::Bool(true)),
            Value::Scaler(Scaler::I32(10)),
        ]);
        let composite = Composite::Vector(array);
        let yaml = serde_yaml::to_string(&composite).unwrap();
        assert!(yaml.contains("true"));
        assert!(yaml.contains("10"));
    }
}
