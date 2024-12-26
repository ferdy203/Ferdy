use super::source_config::GatewayConfig;

#[derive(Debug, Clone)]
pub struct DakiaConfig {
    pub dp: String,
    pub daemon: bool,
    pub error_log: String,
    pub pid_file: String,
    pub upgrade_sock: String,
    pub user: Option<String>,
    pub group: Option<String>,
    pub threads: usize,
    pub work_stealing: bool,
    pub grace_period_seconds: Option<u64>,
    pub graceful_shutdown_timeout_seconds: Option<u64>,
    pub upstream_keepalive_pool_size: usize,
    pub upstream_connect_offload_threadpools: Option<usize>,
    pub upstream_connect_offload_thread_per_pool: Option<usize>,
    pub upstream_debug_ssl_keylog: bool,
    pub gateways: Vec<GatewayConfig>,
}

impl Default for DakiaConfig {
    fn default() -> Self {
        Self {
            dp: Default::default(),
            daemon: Default::default(),
            error_log: Default::default(),
            pid_file: Default::default(),
            upgrade_sock: Default::default(),
            user: Default::default(),
            group: Default::default(),
            threads: Default::default(),
            work_stealing: Default::default(),
            grace_period_seconds: Default::default(),
            graceful_shutdown_timeout_seconds: Default::default(),
            upstream_keepalive_pool_size: Default::default(),
            upstream_connect_offload_threadpools: Default::default(),
            upstream_connect_offload_thread_per_pool: Default::default(),
            upstream_debug_ssl_keylog: Default::default(),
            gateways: Default::default(),
        }
    }
}
