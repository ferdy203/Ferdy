use crate::config::args::DakiaArgs;
use crate::config::router;
use crate::libs::utils::get_or_default;
use log::{debug, error, warn};
use pingora::{prelude::Opt, server::configuration::ServerConf};
use serde;
use serde_yaml;
use std::{
    fs::{self},
    path::Path,
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DakiaRawConfig {
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
    pub router: Option<router::RouterConfig>,
}

pub struct DakiaConfigTemp {
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
    pub router: Option<router::RouterConfig>,
}

impl DakiaRawConfig {
    fn to_dakia_config(&self) -> DakiaConfigTemp {
        DakiaConfigTemp {
            dp: "/etc/dakia".to_string(),
            daemon: get_or_default(self.daemon, false),
            error_log: get_or_default(
                self.error_log.clone(),
                "/var/log/dakia/error.log".to_string(),
            ),
            pid_file: get_or_default(self.pid_file.clone(), "/var/run/dakia.pid".to_string()),
            upgrade_sock: get_or_default(
                self.upgrade_sock.clone(),
                "/var/run/dakia_upgrade.sock".to_string(),
            ),
            user: self.user.clone(),
            group: self.group.clone(),
            threads: get_or_default(self.threads, 1),
            work_stealing: get_or_default(self.work_stealing, true),
            grace_period_seconds: self.grace_period_seconds,
            graceful_shutdown_timeout_seconds: self.graceful_shutdown_timeout_seconds,
            upstream_keepalive_pool_size: get_or_default(self.upstream_keepalive_pool_size, 128),
            upstream_connect_offload_threadpools: self.upstream_connect_offload_threadpools,
            upstream_connect_offload_thread_per_pool: self.upstream_connect_offload_thread_per_pool,
            upstream_debug_ssl_keylog: get_or_default(self.upstream_debug_ssl_keylog, false),
            router: self.router.clone(),
        }
    }
}

impl Default for DakiaRawConfig {
    fn default() -> Self {
        DakiaRawConfig {
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
            router: None,
        }
    }
}

impl DakiaConfigTemp {
    pub fn to_pingora_opt(&self) -> Opt {
        let mut opt = Opt::default();
        opt.daemon = self.daemon;
        opt.conf = Some(self.dp.clone() + "/config/pingora.yaml");
        opt
    }

    pub fn to_pingora_server_config(&self) -> ServerConf {
        // TODO: use Arc instead of clone
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

    pub fn build(args: &DakiaArgs) -> Self {
        let dp = match &args.dp {
            Some(dp) => dp,
            None => "/etc/dakia",
        };

        let cp = Path::new(dp).join("config/dakia.yaml");

        let is_dakia_config_file_readable = match fs::metadata(&cp) {
            Ok(metadata) => metadata.is_file(),
            Err(e) => {
                if args.dp.is_some() {
                    error!("Failed to load Dakia config file. The file might be missing, inaccessible, or malformed: {:?}", e);
                }

                false
            }
        };

        let dakia_raw_config_from_file = if is_dakia_config_file_readable {
            // TODO: handle unwrap() here
            let raw_config = fs::read_to_string(&cp).unwrap();

            let dakia_raw_config_from_file: DakiaRawConfig =
                serde_yaml::from_str(&raw_config).unwrap();

            debug!(
                "\n========== Dakia Config ==========\n{:#?}\n===================================",
                dakia_raw_config_from_file
            );
            dakia_raw_config_from_file
        } else {
            let default_dakia_raw_config = DakiaRawConfig::default();
            warn!(
                "⚠️  Config File Not Found!\n👉 Using Default Configuration\n {:#?}",
                default_dakia_raw_config
            );
            default_dakia_raw_config
        };

        // write server config to file for pingora to read

        let mut dakia_config = dakia_raw_config_from_file.to_dakia_config();
        dakia_config.dp = dp.to_string();

        dakia_config
    }
}
