pub trait PatternMatcher: Send + Sync {
    fn is_match(&self, text: str) -> bool;
}
