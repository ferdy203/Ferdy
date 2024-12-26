use std::{fs, path::Path};

use log::{debug, warn};
use pingora::{prelude::Opt, server::configuration::ServerConf};

use crate::{
    config::source_config::SourceDakiaRawConfig,
    error::{DakiaError, ImmutStr},
    shared::get_or_default,
    shared::IntoRef,
};

use super::{source_config::GatewayConfig, DakiaArgs};

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

impl DakiaConfig {
    pub fn from_args(args: DakiaArgs) -> Result<Self, Box<DakiaError>> {
        let dp = args.dp.as_deref().unwrap_or("/etc/dakia"); // dakia path
        let cp = Path::new(dp).join("config/dakia.yaml"); // configs path

        let is_dakia_config_file_readable = fs::metadata(&cp)
            .map(|metadata| metadata.is_file())
            .unwrap_or(false);

        if args.dp.is_some() && !is_dakia_config_file_readable {
            let e = DakiaError::create(
                crate::error::ErrorType::InternalError,
                crate::error::ErrorSource::Internal,
                Some(ImmutStr::from("Failed to load Dakia config file. The file might be missing, inaccessible, or malformed!")),
                None,
            );
            return Err(e);
        }

        let dakia_config_raw_str = if is_dakia_config_file_readable {
            let raw_config = fs::read_to_string(&cp).map_err(|e| DakiaError::create(
                crate::error::ErrorType::InternalError,
                crate::error::ErrorSource::Internal,
                Some(ImmutStr::from("Failed to load Dakia config file. The file might be missing, inaccessible, or malformed!")),
                Some(Box::new(e)),
            ))?;

            let dakia_source_config: SourceDakiaRawConfig = serde_yaml::from_str(&raw_config)
                .map_err(|e| {
                    DakiaError::create(
                        crate::error::ErrorType::InternalError,
                        crate::error::ErrorSource::Internal,
                        Some(ImmutStr::from("Failed to parse config the file.")),
                        Some(Box::new(e)),
                    )
                })?;

            debug!(
                "\n========== Dakia Config ==========\n{:#?}\n===================================",
                dakia_source_config
            );
            dakia_source_config
        } else {
            let default_dakia_source_config = SourceDakiaRawConfig::default();
            warn!(
                "⚠️  Config File Not Found!\n👉 Using Default Configuration\n {:#?}",
                default_dakia_source_config
            );
            default_dakia_source_config
        };

        Ok(DakiaConfig::from(dakia_config_raw_str))
    }
}

impl From<SourceDakiaRawConfig> for DakiaConfig {
    fn from(source_dakia_raw_config: SourceDakiaRawConfig) -> Self {
        DakiaConfig {
            dp: "/etc/dakia".to_string(),
            daemon: get_or_default(source_dakia_raw_config.daemon, false),
            error_log: get_or_default(
                source_dakia_raw_config.error_log.clone(),
                "/var/log/dakia/error.log".to_string(),
            ),
            pid_file: get_or_default(
                source_dakia_raw_config.pid_file.clone(),
                "/var/run/dakia.pid".to_string(),
            ),
            upgrade_sock: get_or_default(
                source_dakia_raw_config.upgrade_sock.clone(),
                "/var/run/dakia_upgrade.sock".to_string(),
            ),
            user: source_dakia_raw_config.user.clone(),
            group: source_dakia_raw_config.group.clone(),
            threads: get_or_default(source_dakia_raw_config.threads, 1),
            work_stealing: get_or_default(source_dakia_raw_config.work_stealing, true),
            grace_period_seconds: source_dakia_raw_config.grace_period_seconds,
            graceful_shutdown_timeout_seconds: source_dakia_raw_config
                .graceful_shutdown_timeout_seconds,
            upstream_keepalive_pool_size: get_or_default(
                source_dakia_raw_config.upstream_keepalive_pool_size,
                128,
            ),
            upstream_connect_offload_threadpools: source_dakia_raw_config
                .upstream_connect_offload_threadpools,
            upstream_connect_offload_thread_per_pool: source_dakia_raw_config
                .upstream_connect_offload_thread_per_pool,
            upstream_debug_ssl_keylog: get_or_default(
                source_dakia_raw_config.upstream_debug_ssl_keylog,
                false,
            ),
            gateways: vec![],
        }
    }
}

impl IntoRef<Opt> for DakiaConfig {
    fn into_ref(&self) -> Opt {
        let mut opt = Opt::default();
        opt.daemon = self.daemon;
        opt.conf = Some(self.dp.clone() + "/config/pingora.yaml");
        opt
    }
}

impl IntoRef<ServerConf> for DakiaConfig {
    fn into_ref(&self) -> ServerConf {
        ServerConf {
            daemon: self.daemon,
            error_log: Some(self.error_log.clone()),
            grace_period_seconds: self.grace_period_seconds,
            graceful_shutdown_timeout_seconds: self.graceful_shutdown_timeout_seconds,
            group: self.group.clone(),
            user: self.user.clone(),
            threads: self.threads,
            pid_file: self.pid_file.clone(),
            upgrade_sock: self.upgrade_sock.clone(),
            upstream_connect_offload_thread_per_pool: self.upstream_connect_offload_thread_per_pool,
            upstream_debug_ssl_keylog: self.upstream_debug_ssl_keylog,
            upstream_connect_offload_threadpools: self.upstream_connect_offload_threadpools,
            upstream_keepalive_pool_size: self.upstream_keepalive_pool_size,
            work_stealing: self.work_stealing,
            version: 1,
            ca_file: None,
            client_bind_to_ipv4: vec![],
            client_bind_to_ipv6: vec![],
        }
    }
}
