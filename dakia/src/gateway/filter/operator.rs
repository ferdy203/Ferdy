use crate::shared::pattern_matcher::PatternMatcher;

pub enum RelationalOperator {
    Eq(Vec<u8>),
    Ne(Vec<u8>),
}

pub enum SetOperator {
    In(Vec<Vec<u8>>),
    Nin(Vec<Vec<u8>>),
}

pub enum PatternOperator {
    Contains(Vec<u8>),
    NotContains(Vec<u8>),
    StartsWith(Vec<u8>),
    NotStartWith(Vec<u8>),
    EndsWith(Vec<u8>),
    NotEndsWith(Vec<u8>),
    Matches(Box<dyn PatternMatcher>),
}

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
    Custom(String), // Allows custom headers
}

pub enum CriteriaOperator {
    Relation(RelationalOperator),
    Pettern(PatternOperator),
    Set(SetOperator),
    Exists(bool),
}

pub enum LogicalCriteriaOperator {
    And(Vec<CriteriaOperator>),
    Or(Vec<CriteriaOperator>),
}

pub enum PartCriteriaOperator {
    CriteriaOperator(CriteriaOperator),
    LogicalCriteriaOperator(LogicalCriteriaOperator),
}

struct HeaderCriteria {
    name: Header,
    operator: PartCriteriaOperator,
}

struct QueryCriteria {
    name: Vec<u8>,
    operator: PartCriteriaOperator,
}

struct CookieCriteria {
    name: Vec<u8>,
    operator: PartCriteriaOperator,
}

pub enum PartFilterCriteria {
    Header(HeaderCriteria),
    Query(QueryCriteria),
    Cookie(CookieCriteria),
    Path(PartCriteriaOperator),
    Scheme(PartCriteriaOperator),
    Method(PartCriteriaOperator),
}

pub enum FilterCriteria {
    Logical(LogicalFilterCriteria),
    PartFilterCriteria(PartFilterCriteria),
}

pub enum LogicalFilterCriteria {
    And(Vec<PartFilterCriteria>),
    Or(Vec<PartFilterCriteria>),
}

pub struct Filter {
    criteria: Vec<FilterCriteria>,
}
