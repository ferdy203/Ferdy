use crate::config::DakiaConfig;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::{Mutex, MutexGuard, PoisonError};

pub struct ConfigManager {
    configs: AtomicPtr<HashMap<i64, Arc<DakiaConfig>>>,
    version: i64,
    mutex: Mutex<()>,
}

pub static mut CONFIG_MANAGER: Lazy<ConfigManager> = Lazy::new(|| ConfigManager::new());

// https://stackoverflow.com/questions/77547984/relation-of-mutex-and-cpu-caches-and-memory-fences
impl ConfigManager {
    pub fn new() -> Self {
        ConfigManager {
            configs: AtomicPtr::new(Box::into_raw(Box::new(HashMap::new()))),
            version: 0,
            mutex: Mutex::new(()),
        }
    }

    pub fn add_config(
        &mut self,
        new_config: DakiaConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _g = match self.mutex.lock() {
            Ok(guard) => guard,
            Err(e) => {
                return Err(self.get_mutex_lock_fail_error(e));
            }
        };

        self.version = self.version + 1;
        let configs_ptr = self.configs.load(Ordering::SeqCst);

        unsafe {
            let configs = &mut *configs_ptr;
            configs.insert(self.version, Arc::new(new_config));
        };

        Ok(())
    }

    // Get a config by its version
    pub fn get_config(&self, version: i64) -> Result<Arc<DakiaConfig>, Box<dyn std::error::Error>> {
        if version < 0 {
            return Err(self.get_config_not_found_error());
        }

        let configs_ptr = self.configs.load(Ordering::SeqCst);

        unsafe {
            let configs = &*configs_ptr;
            let config = configs.get(&version);

            if let Some(dakia_config) = config {
                return Ok(dakia_config.clone());
            }
        }

        let _g = match self.mutex.lock() {
            Ok(guard) => guard,
            Err(e) => {
                return Err(self.get_mutex_lock_fail_error(e));
            }
        };

        let configs_ptr = self.configs.load(Ordering::SeqCst);
        unsafe {
            let configs = &mut *configs_ptr;
            let config = configs.get(&version);
            if let Some(dakia_config) = config {
                return Ok(dakia_config.clone());
            }
        };

        // release lock to avoid deadlock
        drop(_g);
        self.get_config(version - 1)
    }

    pub fn get_latest_config(&self) -> Result<Arc<DakiaConfig>, Box<dyn std::error::Error>> {
        self.get_config(self.version)
    }

    fn get_mutex_lock_fail_error(
        &self,
        e: PoisonError<MutexGuard<()>>,
    ) -> Box<dyn std::error::Error> {
        Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Can not retrive the config {}", e),
        ))
    }

    fn get_config_not_found_error(&self) -> Box<dyn std::error::Error> {
        Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Can not retrive the config"),
        ))
    }
}
