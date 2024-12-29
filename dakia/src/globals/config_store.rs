use once_cell::sync::Lazy;

use crate::{
    config::DakiaConfig,
    error::{DakiaError, DakiaResult, VoidDakiaResult},
};
use std::sync::{Arc, RwLock};

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

// https://stackoverflow.com/questions/77547984/relation-of-mutex-and-cpu-caches-and-memory-fences
// TODO: We can use crossbeam epoch, if epoch::Guard can be kept across threads because of tokio run time
// as per doc epoch::Guard is pinned to a thread, so keeping it across threads could lead to undefined behaviour
//
// TODO: explore lock free data structure to improve the performance of cofig read/write
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

pub fn get() -> DakiaResult<Arc<DakiaConfig>> {
    let read_guard = CONFIG_STORE
        .read()
        .map_err(|_| DakiaError::create_internal())?;
    let config = read_guard.get_config();
    Ok(config)
}

pub fn store(config: DakiaConfig) -> VoidDakiaResult {
    let mut write_guard = CONFIG_STORE
        .write()
        .map_err(|_| DakiaError::create_internal())?;
    write_guard.store_config(config);
    Ok(())
}

pub fn inner() -> DakiaResult<DakiaConfig> {
    let config = get()?;
    Ok((*config).clone())
}
