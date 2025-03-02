use crate::error::{DakiaError, DakiaResult};

use super::query::{Composite, Map, Operator, Query, Scaler, SupplierValue, Value};

static OPERATOR_IDENTIFIRE: &str = "$";
static LOGICAL_OPERATOR: [Operator; 2] = [Operator::And, Operator::Or];

fn match_str(operator: &Operator, qval: &str, sval: &str) -> DakiaResult<bool> {
    let matched = match operator {
        Operator::Eq => qval == sval,
        Operator::Ne => qval != sval,
        Operator::Contains => sval.contains(qval),
        Operator::NotContains => !sval.contains(qval),
        Operator::StartsWith => sval.starts_with(qval),
        Operator::NotStartWith => !sval.starts_with(qval),
        Operator::EndsWith => sval.ends_with(qval),
        Operator::NotEndsWith => !sval.ends_with(qval),
        Operator::Matches => {
            // TODO: create regex registry after once dakia will be moved to shared nothing arch..
            // registry will allow us to reuse the same compiled regex multiple times throughout the application
            // make sure to consider thread sefty
            let regex = pcre2::bytes::Regex::new(qval)?;
            regex.is_match(sval.as_bytes())?
        }
        _ => {
            return Err(DakiaError::i_explain(format!(
                "Invalid operator {operator:?} for string {qval}"
            )))
        }
    };

    Ok(matched)
}

fn match_int<T, U>(operator: &Operator, qval: T, sval: U) -> DakiaResult<bool>
where
    T: PartialOrd<U>,
{
    let result = match operator {
        Operator::Eq => qval == sval,
        Operator::Ne => qval != sval,
        _ => {
            return Err(DakiaError::i_explain(format!(
                "Invalid operator {operator:?} for integer argumetns "
            )))
        }
    };

    Ok(result)
}

fn match_bool(operator: &Operator, qval: bool, sval: &SupplierValue) -> DakiaResult<bool> {
    match operator {
        Operator::Exists => {
            if let SupplierValue::None = sval {
                return Ok(!qval); // not exists
            } else {
                return Ok(qval); // exists
            }
        }
        _ => {
            return Err(DakiaError::i_explain(format!(
                "Invalid operator {operator:?} for boolean argumetns "
            )))
        }
    }
}

fn is_val_in_vec(vec: &Vec<Value>, sval: &SupplierValue) -> DakiaResult<bool> {
    for val in vec.iter() {
        let matched = exec_operator(&Operator::Eq, val, sval)?;
        if matched {
            return Ok(true);
        }
    }
    Ok(false)
}

fn match_vec(operator: &Operator, vec: &Vec<Value>, sval: &SupplierValue) -> DakiaResult<bool> {
    let present = is_val_in_vec(vec, sval)?;
    let result = match operator {
        Operator::In => present,
        Operator::Nin => !present,
        _ => {
            return Err(DakiaError::i_explain(format!(
                "Invalid operator {operator:?} for array type {vec:?}!"
            )))
        }
    };

    Ok(result)
}

fn exec_operator(operator: &Operator, qval: &Value, sval: &SupplierValue) -> DakiaResult<bool> {
    match qval {
        Value::Scaler(scaler_val) => match scaler_val {
            Scaler::String(qstr) => match sval {
                SupplierValue::Str(sstr) => match_str(operator, &qstr, &sstr),
                _ => Err(DakiaError::i_explain(format!(
                    "expected string and found {sval:?} !"
                ))),
            },
            Scaler::I32(qint) => match sval {
                SupplierValue::I32(sint) => match_int(operator, qint, sint),
                _ => Err(DakiaError::i_explain(format!(
                    "expected integer and found {sval:?} !"
                ))),
            },
            Scaler::Bool(b) => match_bool(operator, *b, sval),
        },
        Value::Composite(composite_val) => match composite_val {
            Composite::Map(_) => Err(DakiaError::i_explain(format!("{qval:?} can not be map!"))),
            Composite::Vector(vec) => match_vec(operator, vec, sval),
        },
    }
}

fn exec_omap<'a, F>(path: &'a str, omap: &Map, supplier: &F) -> DakiaResult<bool>
where
    F: Fn(&'a str) -> DakiaResult<SupplierValue<'a>>,
{
    let sval = supplier(&path)?;
    for (okey, qval) in omap.iter() {
        let operator = Operator::try_from(okey.as_str())?;
        if LOGICAL_OPERATOR.contains(&operator) {
            return Err(DakiaError::i_explain(
                "nested logical operator is not supported.",
            ));
        }

        let matched = exec_operator(&operator, qval, &sval)?;
        if !matched {
            return Ok(false);
        }
    }

    Ok(true)
}

fn exec_logical<'a, F>(query_map: &'a Map, and: bool, supplier: &F) -> DakiaResult<bool>
where
    F: Fn(&'a str) -> DakiaResult<SupplierValue<'a>>,
{
    for (key, qval) in query_map.iter() {
        let matched = if key.starts_with(OPERATOR_IDENTIFIRE) {
            // TODO: add support for nested logical operator - https://github.com/rust-lang/rust/issues/43520
            return Err(DakiaError::i_explain(format!(
                "Support for nested logical operator is not yet available"
            )));

            // let operator = Operator::try_from(key.as_str())?;
            // if !LOGICAL_OPERATOR.contains(&operator) {
            //     return Err(DakiaError::i_explain(
            //         "No operator should appear directly after logical operator.",
            //     ));
            // }

            // // value must be operator map because it's logical
            // match qval {
            //     Value::Composite(Composite::Map(omap)) => {
            //         exec_logical(omap, matches!(operator, Operator::And), &supplier)
            //     }
            //     _ => Err(DakiaError::i_explain(format!(
            //         "a map was expected, found {qval:?}"
            //     ))),
            // }?
        } else {
            match qval {
                Value::Scaler(_) => {
                    let sval = supplier(key.as_str())?;
                    exec_operator(&Operator::Eq, qval, &sval)?
                }
                Value::Composite(Composite::Map(omap)) => exec_omap(&key, omap, supplier)?,
                _ => {
                    return Err(DakiaError::i_explain(format!(
                        "Invalid value {qval:?} found for key {key}"
                    )))
                }
            }
        };

        // if operator is $and then return false if any match is false
        if and && !matched {
            return Ok(false);
        }
        // if operator is $or then return if any match is true
        else if matched {
            return Ok(true);
        }
    }

    Ok(true)
}

pub fn exec<'a, F>(query: &'a Query, supplier: F) -> DakiaResult<bool>
where
    F: Fn(&'a str) -> DakiaResult<SupplierValue<'a>>,
{
    for (key, qval) in query.iter() {
        let matched = if key.starts_with(OPERATOR_IDENTIFIRE) {
            let operator = Operator::try_from(key.as_str())?;

            // only logical operator can be specified at the root level
            if !LOGICAL_OPERATOR.contains(&operator) {
                return Err(DakiaError::i_explain(format!(
                    "expected $or or $and, found {operator:?}!"
                )));
            }

            // value must be operator map because it's logical
            match qval {
                Value::Composite(Composite::Map(omap)) => {
                    exec_logical(omap, matches!(operator, Operator::And), &supplier)
                }
                _ => Err(DakiaError::i_explain(format!(
                    "a map was expected, found {qval:?}"
                ))),
            }?
        } else {
            match qval {
                Value::Scaler(_) => {
                    let sval = supplier(key.as_str())?;
                    exec_operator(&Operator::Eq, qval, &sval)?
                }
                Value::Composite(Composite::Map(omap)) => exec_omap(&key, omap, &supplier)?,
                _ => {
                    return Err(DakiaError::i_explain(format!(
                        "Invalid value {qval:?} found for key {key}"
                    )))
                }
            }
        };

        if !matched {
            return Ok(false);
        }
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[derive(Debug, PartialEq)]
    struct Query {
        field: String,
        operator: Operator,
        value: Value,
    }

    impl Query {
        fn new(field: &str, operator: Operator, value: Value) -> Self {
            Query {
                field: field.to_string(),
                operator,
                value,
            }
        }

        fn exec(&self, data: &HashMap<String, Value>) -> bool {
            match data.get(&self.field) {
                Some(actual_value) => match self.operator {
                    Operator::Eq => actual_value == &self.value,
                    Operator::Ne => actual_value != &self.value,
                    Operator::Exists => data.contains_key(&self.field),
                    _ => false, // Extend for other operators
                },
                None => false,
            }
        }
    }

    #[test]
    fn test_query_execution() {
        let mut data = HashMap::new();
        data.insert(
            "name".to_string(),
            Value::Scaler(Scaler::String("Alice".to_string())),
        );

        let query = Query::new(
            "name",
            Operator::Eq,
            Value::Scaler(Scaler::String("Alice".to_string())),
        );
        assert!(query.exec(&data));

        let query = Query::new(
            "name",
            Operator::Ne,
            Value::Scaler(Scaler::String("Bob".to_string())),
        );
        assert!(query.exec(&data));

        let query = Query::new("age", Operator::Exists, Value::Scaler(Scaler::I32(25)));
        assert!(!query.exec(&data)); // "age" doesn't exist in the data
    }
}
