use crate::error::BError;
use crate::error::DakiaError;
use crate::qe::query::Query;

use super::DownstreamConfig;
use super::InetAddress;
use super::RouterConfig;
use super::UpstreamConfig;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GatewayConfig {
    pub name: String, // TODO: use auto generated name
    // TODO: add type = HTTP, TCP, SMTP, etc
    pub bind_addresses: Vec<InetAddress>,
    pub downstreams: Vec<DownstreamConfig>,
    pub upstreams: Vec<UpstreamConfig>,

    #[serde(default)]
    pub routers: Vec<RouterConfig>,
}

impl GatewayConfig {
    pub fn find_router_config<F>(&self, predicate: F) -> Option<&RouterConfig>
    where
        F: Fn(&Query) -> bool,
    {
        self.routers
            .iter()
            .find(|router_config| match &router_config.filter {
                None => true,
                Some(filter) => predicate(filter),
            })
    }

    pub fn find_router_config_or_err<F>(&self, predicate: F) -> Result<&RouterConfig, BError>
    where
        F: Fn(&Query) -> bool,
    {
        let router_config = self.find_router_config(predicate);
        router_config.ok_or(DakiaError::create_unknown_context(
            crate::error::ImmutStr::Static("router config not found".into()),
        ))
    }

    pub fn find_default_upstream(&self) -> Option<&UpstreamConfig> {
        self.upstreams
            .iter()
            .find(|upstream_config| upstream_config.default)
    }

    pub fn find_upstream_config(
        &self,
        upstream_name: &str,
        fallback: bool,
    ) -> Option<&UpstreamConfig> {
        let upstream_config = self
            .upstreams
            .iter()
            .find(|upstream_config| upstream_config.name == upstream_name);
        match upstream_config {
            Some(upstream_config) => Some(upstream_config),
            None => {
                if fallback {
                    self.find_default_upstream()
                } else {
                    None
                }
            }
        }
    }

    pub fn find_upstream_config_or_err(
        &self,
        upstream_name: &str,
        fallback: bool,
    ) -> Result<&UpstreamConfig, BError> {
        self.find_upstream_config(upstream_name, fallback).ok_or(
            DakiaError::create_unknown_context(crate::error::ImmutStr::Static(
                "upstream config not found".into(),
            )),
        )
    }
}
