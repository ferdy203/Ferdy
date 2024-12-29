use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::config::DakiaConfig;
use std::sync::Arc;

pub struct GlobalConfigStore {
    config: Arc<DakiaConfig>,
}

static CONFIG_STORE: Lazy<RwLock<GlobalConfigStore>> =
    Lazy::new(|| RwLock::new(GlobalConfigStore::new()));

pub trait ConfigStore: Send + Sync {
    fn store_config(&mut self, new_config: DakiaConfig) -> ();
    fn get_config(&self) -> Arc<DakiaConfig>;
    fn get_inner(&self) -> DakiaConfig;
}

impl GlobalConfigStore {
    pub fn from(dakia_config: &DakiaConfig) -> Self {
        GlobalConfigStore {
            config: Arc::new(dakia_config.clone()),
        }
    }

    pub fn new() -> Self {
        Self {
            config: Arc::new(DakiaConfig::default()),
        }
    }
}

impl ConfigStore for GlobalConfigStore {
    fn store_config(&mut self, new_config: DakiaConfig) -> () {
        let mut cloned_config = new_config.clone();
        cloned_config.version = new_config.version + 1;

        self.config = Arc::new(cloned_config);
    }

    fn get_config(&self) -> Arc<DakiaConfig> {
        self.config.clone()
    }

    fn get_inner(&self) -> DakiaConfig {
        let arc_config = self.get_config();
        (*arc_config).clone()
    }
}

pub async fn get() -> Arc<DakiaConfig> {
    let read_guard = CONFIG_STORE.read().await;
    read_guard.get_config()
}

pub async fn store(config: DakiaConfig) -> () {
    let mut write_guard = CONFIG_STORE.write().await;
    write_guard.store_config(config);
}

pub async fn inner() -> DakiaConfig {
    let config = get().await;
    (*config).clone()
}
