use crate::error::{DakiaError, DakiaResult};

use super::query::{Composite, Map, Operator, Query, Scaler, SupplierValue, Value};
static OPERATOR_IDENTIFIRE: &str = "$";
static LOGICAL_OPERATOR: [Operator; 2] = [Operator::And, Operator::Or];
static SCALER_OPERATOR: [Operator; 9] = [
    Operator::Eq,
    Operator::Ne,
    Operator::Contains,
    Operator::NotContains,
    Operator::StartsWith,
    Operator::NotStartWith,
    Operator::EndsWith,
    Operator::NotEndsWith,
    Operator::Matches,
];
static ARRAY_OPERATOR: [Operator; 2] = [Operator::In, Operator::Nin];

fn match_str(operator: &Operator, query_str: &str, supplier_str: &str) -> DakiaResult<bool> {
    let matched = match operator {
        Operator::Eq => query_str == supplier_str,
        Operator::Ne => query_str != supplier_str,
        Operator::Contains => supplier_str.contains(query_str),
        Operator::NotContains => !supplier_str.contains(query_str),
        Operator::StartsWith => supplier_str.starts_with(query_str),
        Operator::NotStartWith => !supplier_str.starts_with(query_str),
        Operator::EndsWith => supplier_str.ends_with(query_str),
        Operator::NotEndsWith => !supplier_str.ends_with(query_str),
        Operator::Matches => {
            // TODO: create regex registry after once dakia will be moved to shared nothing arch..
            // registry will allow here to reuse the same compiled regex multiple throughout the application
            let regex = pcre2::bytes::Regex::new(query_str)?;
            regex.is_match(supplier_str.as_bytes())?
        }
        _ => {
            return Err(DakiaError::i_explain(format!(
                "Invalid operator {operator:?} for string {query_str}"
            )))
        }
    };

    Ok(matched)
}

fn match_int<T, U>(operator: &Operator, a: T, b: U) -> DakiaResult<bool>
where
    T: PartialOrd<U>,
{
    let result = match operator {
        Operator::Eq => a == b,
        Operator::Ne => a != b,
        _ => {
            return Err(DakiaError::i_explain(format!(
                "Invalid operator {operator:?} for integer argumetns "
            )))
        }
    };

    Ok(result)
}

fn is_val_in_vec(ar: &Vec<Value>, supplier_val: &SupplierValue) -> DakiaResult<bool> {
    for val in ar.iter() {
        let matched = exec_operator(&Operator::Eq, val, supplier_val)?;
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
                SupplierValue::I32(sint) => Err(DakiaError::i_explain(format!(
                    "expected string and found {sint:?} !"
                ))),
                SupplierValue::Str(sstr) => match_str(operator, &qstr, &sstr),
            },
            Scaler::I32(qint) => match sval {
                SupplierValue::I32(sint) => match_int(operator, qint, sint),
                SupplierValue::Str(sstr) => Err(DakiaError::i_explain(format!(
                    "expected integer and found {sstr:?} !"
                ))),
            },
        },
        Value::Composite(composite_val) => match composite_val {
            Composite::Map(_) => Err(DakiaError::i_explain(format!("{qval:?} can not be map!"))),
            Composite::Array(vec) => match_vec(operator, vec, sval),
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
    for (key, value) in query_map.iter() {
        if key.starts_with(OPERATOR_IDENTIFIRE) {
            return Err(DakiaError::i_explain(
                "No operator should appear directly after logical operator.",
            ));
        }

        // scaler values are matched like $eq operator
        if let Value::Scaler(_) = value {
            let sval = supplier(key.as_str())?;
            let matched = exec_operator(&Operator::Eq, value, &sval)?;
            // if operator is $and then return false if any match is false
            if and && !matched {
                return Ok(false);
            }
            // if operator is $or then return true if any match is true
            else if matched {
                return Ok(true);
            }
        }

        // value must be operator map
        let matched = match value {
            Value::Composite(Composite::Map(omap)) => exec_omap(&key, omap, supplier),
            _ => Err(DakiaError::i_explain(format!(
                "a map was expected, found {value:?}"
            ))),
        }?;

        // if operator is $and then return false if any match is false
        if and && !matched {
            return Ok(false);
        }
        // if operator is $or then return if any match is true
        else if matched {
            return Ok(true);
        }
    }
    Ok(false)
}

pub fn exec<'a, F>(query: &'a Query, supplier: F) -> DakiaResult<bool>
where
    F: Fn(&'a str) -> DakiaResult<SupplierValue<'a>>,
{
    for (key, qval) in query.iter() {
        if key.starts_with(OPERATOR_IDENTIFIRE) {
            let operator = Operator::try_from(key.as_str())?;

            // only logical operator can be specified at the root level
            if !LOGICAL_OPERATOR.contains(&operator) {
                return Err(DakiaError::i_explain(format!(
                    "expected $or or $and, found {operator:?}!"
                )));
            }

            // value must be operator map because it's logical
            let matched = match qval {
                Value::Composite(Composite::Map(omap)) => {
                    exec_logical(omap, matches!(operator, Operator::And), &supplier)
                }
                _ => Err(DakiaError::i_explain(format!(
                    "a map was expected, found {qval:?}"
                ))),
            }?;

            if !matched {
                return Ok(false);
            }
        }

        // scaler values are matched like $eq operator
        if let Value::Scaler(_) = qval {
            let sval = supplier(key.as_str())?;
            let matched = exec_operator(&Operator::Eq, qval, &sval)?;

            if !matched {
                return Ok(false);
            }
        }

        // key is not logical operator, value is not scaler then it must be an operator map
        let matched = match qval {
            Value::Composite(Composite::Map(omap)) => exec_omap(&key, omap, &supplier),
            _ => Err(DakiaError::i_explain(format!(
                "a map was expected, found {qval:?}"
            ))),
        }?;

        if !matched {
            return Ok(false);
        }
    }

    Ok(true)
}
