use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::config::DakiaConfig;
use std::sync::Arc;

pub struct GlobalConfigStore {
    config: Arc<DakiaConfig>,
}

// TODO: remove static global, use any lock free data structure instead
static CONFIG_STORE: Lazy<RwLock<GlobalConfigStore>> =
    Lazy::new(|| RwLock::new(GlobalConfigStore::new()));

pub trait DakiaState: Send + Sync {
    fn update_state(&mut self, new_config: DakiaConfig) -> ();
    fn get_state(&self) -> Arc<DakiaConfig>;
    fn get_inner(&self) -> DakiaConfig;
}

impl GlobalConfigStore {
    pub fn new() -> Self {
        Self {
            config: Arc::new(DakiaConfig::default()),
        }
    }
}

impl DakiaState for GlobalConfigStore {
    fn update_state(&mut self, new_config: DakiaConfig) -> () {
        let mut cloned_config = new_config.clone();
        cloned_config.version = new_config.version + 1;

        self.config = Arc::new(cloned_config);
    }

    fn get_state(&self) -> Arc<DakiaConfig> {
        self.config.clone()
    }

    fn get_inner(&self) -> DakiaConfig {
        let arc_config = self.get_state();
        (*arc_config).clone()
    }
}

pub async fn get() -> Arc<DakiaConfig> {
    let read_guard = CONFIG_STORE.read().await;
    read_guard.get_state()
}

pub async fn store(config: DakiaConfig) -> () {
    let mut write_guard = CONFIG_STORE.write().await;
    write_guard.update_state(config);
}

pub async fn inner() -> DakiaConfig {
    let config = get().await;
    (*config).clone()
}
