mod config_store;
mod regex_registry;

pub use config_store::CONFIG_STORE;
// 4. thread local lazy config read and update

// 1. thread local lazy regex registry
// 2. thread local lazy filter registry
// 3. thread local lazy interceptor registry
