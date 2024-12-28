use crate::config::DakiaConfig;
use crate::error::DakiaError;
use crate::error::ImmutStr;
use log::error;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::{Mutex, MutexGuard, PoisonError};

pub struct ConfigStore {
    configs: AtomicPtr<HashMap<i64, Arc<DakiaConfig>>>,
    version: i64,
    mutex: Mutex<()>,
}

pub static mut CONFIG_STORE: Lazy<ConfigStore> = Lazy::new(|| ConfigStore::new());

// https://stackoverflow.com/questions/77547984/relation-of-mutex-and-cpu-caches-and-memory-fences
// TODO: We can use crossbeam epoch, if epoch::Guard can be kept across threads because of tokio run time
// as per doc epoch::Guard is pinned to a thread, so keeping it across threads could lead to undefined behaviour
//
// TODO: delete old config which is not likely to be used
impl ConfigStore {
    pub fn new() -> Self {
        ConfigStore {
            configs: AtomicPtr::new(Box::into_raw(Box::new(HashMap::new()))),
            version: 0,
            mutex: Mutex::new(()),
        }
    }

    pub fn store_config(&mut self, new_config: DakiaConfig) -> Result<(), Box<DakiaError>> {
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
    pub fn get_config(&self, version: i64) -> Result<Arc<DakiaConfig>, Box<DakiaError>> {
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

    pub fn get_latest_config(&self) -> Result<Arc<DakiaConfig>, Box<DakiaError>> {
        let dc = self.get_config(self.version)?;
        Ok(dc)
    }

    fn get_mutex_lock_fail_error(&self, e: PoisonError<MutexGuard<()>>) -> Box<DakiaError> {
        error!("Error while getting lock in config store {}", e);

        DakiaError::create(
            crate::error::ErrorType::InternalError,
            crate::error::ErrorSource::Internal,
            Some(ImmutStr::Static(
                "Error while getting lock to in config store",
            )),
            None,
        )
    }

    fn get_config_not_found_error(&self) -> Box<DakiaError> {
        DakiaError::create(
            crate::error::ErrorType::InternalError,
            crate::error::ErrorSource::Internal,
            Some(ImmutStr::Static("Can not retrive the config")),
            None,
        )
    }
}
