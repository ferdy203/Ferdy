use super::GatewayConfig;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SourceDakiaRawConfig {
    pub daemon: Option<bool>,
    pub error_log: Option<String>,
    pub pid_file: Option<String>,
    pub upgrade_sock: Option<String>,
    pub user: Option<String>,
    pub group: Option<String>,
    pub threads: Option<usize>,
    pub work_stealing: Option<bool>,
    pub grace_period_seconds: Option<u64>,
    pub graceful_shutdown_timeout_seconds: Option<u64>,
    pub upstream_keepalive_pool_size: Option<usize>,
    pub upstream_connect_offload_threadpools: Option<usize>,
    pub upstream_connect_offload_thread_per_pool: Option<usize>,
    pub upstream_debug_ssl_keylog: Option<bool>,
    pub gateways: Vec<GatewayConfig>,
}

impl Default for SourceDakiaRawConfig {
    // TODO: keep a yaml embeded string for default config with router and interceptors
    fn default() -> Self {
        SourceDakiaRawConfig {
            daemon: None,
            error_log: None,
            pid_file: None,
            upgrade_sock: None,
            user: None,
            group: None,
            threads: None,
            work_stealing: None,
            grace_period_seconds: None,
            graceful_shutdown_timeout_seconds: None,
            upstream_connect_offload_thread_per_pool: None,
            upstream_connect_offload_threadpools: None,
            upstream_debug_ssl_keylog: None,
            upstream_keepalive_pool_size: None,
            gateways: vec![],
        }
    }
}
