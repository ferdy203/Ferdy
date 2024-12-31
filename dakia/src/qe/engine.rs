// take query
// build path of query
// get the value of query from passed closure by passing path
// match and return true/false

use super::query::Query;

// fields of enum SupplierValue should be equivalent to Scaler enum fields of Query
pub enum SupplierValue<'a> {
    I32(i32),
    Str(&'a str),
}

pub fn exec_match<'a, F>(query: &Query, supplier: F) -> bool
where
    F: Fn(&'a str) -> SupplierValue<'a>,
{
    false
}
