use crate::{error::Error, qe::query::Query, shared::pattern_matcher::PatternMatcher};

use super::query2filter::query2filter;

#[derive(Debug)]
pub enum RelationalOperator {
    Eq(Vec<u8>),
    Ne(Vec<u8>),
}

#[derive(Debug)]
pub enum SetOperator {
    In(Vec<Vec<u8>>),
    Nin(Vec<Vec<u8>>),
}

#[derive(Debug)]
pub enum PatternOperator {
    Contains(Vec<u8>),
    NotContains(Vec<u8>),
    StartsWith(Vec<u8>),
    NotStartWith(Vec<u8>),
    EndsWith(Vec<u8>),
    NotEndsWith(Vec<u8>),
    Matches(Box<dyn PatternMatcher>),
}

#[derive(Debug)]
pub enum Header {
    Accept,
    AcceptEncoding,
    AcceptLanguage,
    Authorization,
    CacheControl,
    ContentType,
    ContentLength,
    Cookie,
    Host,
    Origin,
    Referer,
    UserAgent,
    XForwardedFor,
    XRequestId,
    Custom(Vec<u8>), // Allows custom headers
}

impl From<&str> for Header {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "accept" => Header::Accept,
            "accept-encoding" => Header::AcceptEncoding,
            "accept-language" => Header::AcceptLanguage,
            "authorization" => Header::Authorization,
            "cache-control" => Header::CacheControl,
            "content-type" => Header::ContentType,
            "content-length" => Header::ContentLength,
            "cookie" => Header::Cookie,
            "host" => Header::Host,
            "origin" => Header::Origin,
            "referer" => Header::Referer,
            "user-agent" => Header::UserAgent,
            "x-forwarded-for" => Header::XForwardedFor,
            "x-request-id" => Header::XRequestId,
            _ => Header::Custom(value.as_bytes().to_vec()),
        }
    }
}

#[derive(Debug)]
pub enum CriteriaOperator {
    Relation(RelationalOperator),
    Pattern(PatternOperator),
    Set(SetOperator),
    Exists(bool),
}

#[derive(Debug)]
pub enum LogicalCriteriaOperator {
    And(Vec<CriteriaOperator>),
    Or(Vec<CriteriaOperator>),
}

#[derive(Debug)]
pub enum PartCriteriaOperator {
    CriteriaOperator(CriteriaOperator),
    LogicalCriteriaOperator(LogicalCriteriaOperator),
}

#[derive(Debug)]
pub struct HeaderCriteria {
    pub name: Header,
    pub operator: Vec<PartCriteriaOperator>,
}

#[derive(Debug)]
pub struct QueryCriteria {
    pub name: Vec<u8>,
    pub operator: Vec<PartCriteriaOperator>,
}

#[derive(Debug)]
pub struct CookieCriteria {
    pub name: Vec<u8>,
    pub operator: Vec<PartCriteriaOperator>,
}

#[derive(Debug)]
pub enum PartFilterCriteria {
    Header(HeaderCriteria),
    Query(QueryCriteria),
    Cookie(CookieCriteria),
    Path(Vec<PartCriteriaOperator>),
    Scheme(Vec<PartCriteriaOperator>),
    Method(Vec<PartCriteriaOperator>),
}

#[derive(Debug)]
pub enum FilterCriteria {
    Logical(LogicalFilterCriteria),
    PartFilterCriteria(PartFilterCriteria),
}

#[derive(Debug)]
pub enum LogicalFilterCriteria {
    And(Vec<PartFilterCriteria>),
    Or(Vec<PartFilterCriteria>),
}

#[derive(Debug)]
pub struct Filter {
    pub criteria_list: Vec<FilterCriteria>,
}

impl TryFrom<&Query> for Filter {
    type Error = Box<Error>;

    fn try_from(value: &Query) -> Result<Self, Self::Error> {
        query2filter(value)
    }
}
