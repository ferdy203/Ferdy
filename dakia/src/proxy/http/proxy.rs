use crate::{
    config::source_config::GatewayConfig,
    error::{DakiaError, DakiaResult},
    proxy::http::helpers::get_inet_addr_from_backend,
    qe::{engine::exec, query::SupplierValue},
    shared::{config_store, pattern_registry::PatternRegistryType},
};

use super::{
    builder,
    helpers::{self, is_valid_ds_host, part_supplier},
    lb, DakiaHttpGatewayCtx,
};
use async_trait::async_trait;
use pingora::{
    prelude::HttpPeer,
    proxy::{ProxyHttp, Session},
    Error,
};

#[derive(Clone)]
pub struct Proxy {
    name: String,
    ds_host_pattern_registry: PatternRegistryType,
    lb_registry: lb::LbRegistryType,
}

impl Proxy {
    pub async fn build(gateway_config: &GatewayConfig) -> DakiaResult<Proxy> {
        let ds_host_pattern_registry =
            builder::build_ds_host_pattern_registry(gateway_config).await?;
        let lb_registry = builder::build_lb_registry(gateway_config).await?;

        let proxy = Proxy {
            name: gateway_config.name.clone(),
            ds_host_pattern_registry,
            lb_registry,
        };

        Ok(proxy)
    }
}

#[async_trait]
impl ProxyHttp for Proxy {
    type CTX = DakiaHttpGatewayCtx;
    fn new_ctx(&self) -> Self::CTX {
        DakiaHttpGatewayCtx::new()
    }

    async fn early_request_filter(
        &self,
        _session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> Result<(), Box<Error>> {
        // update config into context
        let c = config_store::get().await;
        _ctx.config = c;

        Ok(())
    }

    async fn request_filter(
        &self,
        _session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> Result<bool, Box<Error>> {
        let host = helpers::part_supplier("ds.req.header.host", &_ctx, &_session)?;

        match host {
            SupplierValue::Str(host) => {
                let is_valid_ds_host = is_valid_ds_host(
                    &_ctx.config,
                    &self.name,
                    &self.ds_host_pattern_registry,
                    host,
                )
                .await?;

                if !is_valid_ds_host {
                    // TODO: add option to customize http response status and body
                    helpers::write_response_ds(_session, 403, None).await?;
                    return Ok(true);
                }
            }

            _ => {
                // TODO: add option to customize http response status and body
                helpers::write_response_ds(_session, 400, None).await?;
                return Ok(true);
            }
        };

        Ok(false)
    }

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>, Box<Error>> {
        let gateway_config = _ctx.config.find_gateway_config_or_err(&self.name)?;

        // TODO: return 404 if router config not found
        let router_config = gateway_config.find_router_config_or_err(|filter| {
            exec(filter, |path| part_supplier(path, _ctx, _session))
        })?;

        let upstream_name = &router_config.upstream;
        let mut lb = self.lb_registry.get(&upstream_name).await?;
        lb = match lb {
            None => self.lb_registry.get("default").await?,
            Some(lb) => Some(lb),
        };

        let lb = lb.ok_or(DakiaError::i_explain(format!(
            "load balacer not found for upstream {upstream_name}"
        )))?;

        let backend = lb.select(b"", 256).unwrap(); // hash doesn't matter

        let inet_address = get_inet_addr_from_backend(&backend);

        let upstream_node_config = gateway_config
            .find_upstream_config_or_err(upstream_name, true)
            .map(|a| a.find_upstream_node_config_or_err(inet_address))??;

        let tls = upstream_node_config.tls;
        let sni = upstream_node_config.clone().sni.unwrap_or("".to_string());

        let peer = Box::new(HttpPeer::new(backend.addr, tls, sni));
        Ok(peer)
    }
}
