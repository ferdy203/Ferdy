use pcre2::bytes::Regex;

use crate::error::{BErrorStd, DakiaResult};

pub struct Pcre2PatternMatcher {
    regex: Regex,
}

impl Pcre2PatternMatcher {
    pub fn build(pattern: &str) -> DakiaResult<Self> {
        let pcre2regex = Regex::new(pattern)?;
        let matcher = Self { regex: pcre2regex };
        Ok(matcher)
    }
}

impl PatternMatcher for Pcre2PatternMatcher {
    fn is_match(&self, text: &str) -> Result<bool, BErrorStd> {
        let is_matched = self.regex.is_match(text.as_bytes())?;
        Ok(is_matched)
    }
}

pub trait PatternMatcher: Send + Sync {
    fn is_match(&self, text: &str) -> Result<bool, BErrorStd>;
}
