use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use crate::shared::{PatternMatcher, Registry};

static PATTERN_MACTCHER_STORE: Lazy<RwLock<HashMap<String, Arc<dyn PatternMatcher>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

pub struct PatternRegistry {}

impl Registry<Arc<dyn PatternMatcher>> for PatternRegistry {
    async fn register(&self, key: String, item: Arc<dyn PatternMatcher>) {
        let mut write_guard = PATTERN_MACTCHER_STORE.write().await;
        write_guard.insert(key, item);
    }

    async fn get(&self, key: &str) -> Option<Arc<dyn PatternMatcher>> {
        let read_guard = PATTERN_MACTCHER_STORE.read().await;
        let matcher = read_guard.get(key)?;
        Some(matcher.clone())
    }
}
