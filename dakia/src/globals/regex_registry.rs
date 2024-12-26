use crate::shared::Registry;
use arc_swap::ArcSwap;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;
use std::{collections::HashMap, sync::Arc};

static REGEX_REGISTRY: Lazy<DashMap<String, Arc<String>>> = Lazy::new(DashMap::new);

struct RegexRegistry {}

fn create_arc_string(value: String) -> Arc<String> {
    Arc::new(value)
}

impl Registry<String, String> for RegexRegistry {
    fn register(&self, key: &String, item: &String) -> bool {
        let item_registered = REGEX_REGISTRY.contains_key("key");
        if item_registered {
            return true;
        }

        let arc_item = Arc::new(item.to_string());
        let str_item = item.to_string();

        // let lazy_item: Lazy<Arc<String>, fn() -> Arc<String>> =
        //     Lazy::new(|| Arc::new(create_arc_string(str_item)));

        // let lazy_item: Lazy<Arc<String>, fn() -> Arc<String>> =
        //     Lazy::new(|| Arc::new(item.to_string()));

        let lazy_item: Lazy<Arc<String>, Box<dyn Fn() -> Arc<String>>> =
            Lazy::new(Box::new(move || Arc::new(item.clone())));
        REGEX_REGISTRY.insert(key.to_string(), Arc::new("data".to_string()));
        false
    }

    fn get(&self, key: &String) -> Option<&String> {
        todo!()
    }
}

// filter ->
// interceptor ->
// plugin ->
