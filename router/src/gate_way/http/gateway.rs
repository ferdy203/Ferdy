use super::DakiaHttpGatewayCtx;
use crate::{
    config::{GatewayConfig, UpstreamConfig, UpstreamNodeConfig},
    libs::{pingora::get_header_value, utils::get_or_default},
};
use async_trait::async_trait;
use pingora::{
    http::RequestHeader,
    prelude::HttpPeer,
    proxy::{ProxyHttp, Session},
    Error,
};

use std::collections::HashMap;
use wildmatch::WildMatch;

struct RoutePatternBackendMatcher {
    pattern: WildMatch,
    backend: String,
}

impl RoutePatternBackendMatcher {
    fn is_matched(&self, path: &str) -> bool {
        self.pattern.matches(path)
    }

    fn get_backend(&self) -> &str {
        &self.backend
    }
}

pub struct DakiaHttpGateway {
    downstream_patterns: Vec<WildMatch>,
    route_pattern_backend_matchers: Vec<RoutePatternBackendMatcher>,
    backend_map: HashMap<String, UpstreamConfig>,
    default_backend: Option<UpstreamConfig>,
}

impl DakiaHttpGateway {
    fn get_hosts(gate_way: &GatewayConfig) -> Vec<WildMatch> {
        gate_way
            .downstreams
            .iter()
            .map(|inet_address| {
                let host = &inet_address.host;
                let port = &inet_address.port;

                let host_port = match port {
                    Some(port) => format!("{}:{}", host, port),
                    None => host.to_string(),
                };

                WildMatch::new(&host_port)
            })
            .collect()
    }

    fn get_path_map(gate_way: &GatewayConfig) -> Vec<RoutePatternBackendMatcher> {
        let mut path_backend_list: Vec<RoutePatternBackendMatcher> = vec![];

        gate_way.routers.iter().for_each(|loc| {
            if let Some(route) = &loc.route {
                let path_backend = RoutePatternBackendMatcher {
                    pattern: WildMatch::new(&route.route),
                    backend: loc.upstream.to_string(),
                };
                path_backend_list.push(path_backend);
            }
        });

        path_backend_list
    }

    fn get_backend_map(gate_way: &GatewayConfig) -> HashMap<String, UpstreamConfig> {
        let mut backend_map = HashMap::new();
        gate_way.upstreams.iter().for_each(|backend| {
            backend_map.insert(backend.name.to_string(), backend.clone());
        });
        backend_map
    }

    fn get_default_backend(gate_way: &GatewayConfig) -> Option<UpstreamConfig> {
        let default_backend_ref = gate_way.upstreams.iter().find(|backend| backend.default);

        match default_backend_ref {
            Some(backend_ref) => Some(backend_ref.clone()),
            None => None,
        }
    }

    pub fn build(gate_way: &GatewayConfig) -> DakiaHttpGateway {
        DakiaHttpGateway {
            downstream_patterns: DakiaHttpGateway::get_hosts(gate_way),
            route_pattern_backend_matchers: DakiaHttpGateway::get_path_map(gate_way),
            backend_map: DakiaHttpGateway::get_backend_map(gate_way),
            default_backend: DakiaHttpGateway::get_default_backend(gate_way),
        }
    }

    pub fn get_upstream_config(&self, host: &str, path: String) -> Option<&UpstreamNodeConfig> {
        if !self.is_host_exists(host) {
            return None;
        }

        let backend_name = self.get_path_backend(&path);

        let backend = match backend_name {
            None => None,
            Some(backend_name) => self.get_backend(backend_name),
        };

        match backend {
            Some(backend) => backend.get_upstream_node_config(),
            None => match &self.default_backend {
                Some(default_backend) => default_backend.get_upstream_node_config(),
                None => None,
            },
        }
    }

    fn is_host_exists(&self, host: &str) -> bool {
        self.downstream_patterns
            .iter()
            .any(|wild_host| wild_host.matches(host))
    }

    fn get_path_backend(&self, path: &String) -> Option<&str> {
        // unwrap used here because it'll be always called if path exists
        let path_backend = self
            .route_pattern_backend_matchers
            .iter()
            .find(|path_backend| path_backend.is_matched(path));

        match path_backend {
            Some(pb) => Some(pb.get_backend()),
            None => None,
        }
    }

    fn get_backend(&self, backend_name: &str) -> Option<&UpstreamConfig> {
        self.backend_map.get(backend_name)
    }
}

#[async_trait]
impl ProxyHttp for DakiaHttpGateway {
    type CTX = DakiaHttpGatewayCtx;

    fn new_ctx(&self) -> Self::CTX {
        DakiaHttpGatewayCtx::new()
    }

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        _ctx: &mut DakiaHttpGatewayCtx,
    ) -> Result<Box<HttpPeer>, Box<Error>> {
        let header = get_header_value(_session, "host".to_string())?;

        let header_value = match header {
            None => return Err(Error::new(pingora::ErrorType::ConnectNoRoute)),
            // TODO:  fix data copy issue, data copy using header_value.to_string() needed here to avoid borrowing issue, because header_value is used after brrowing header_value
            Some(header_value) => header_value.to_string(),
        };

        let path: &str = _session.req_header().uri.path();
        let upstream_config = self.get_upstream_config(&header_value, path.to_string());

        match upstream_config {
            Some(upstream_config) => {
                let address = upstream_config.address.get_formatted_address();

                // TODO: fix .clone() here use Box/Rc
                _ctx.upstream_config = Some(upstream_config.clone());

                let peer = Box::new(HttpPeer::new(
                    address,
                    upstream_config.tls,
                    // TODO: avoid clone here
                    get_or_default(upstream_config.sni.clone(), "".to_string()),
                ));
                Ok(peer)
            }
            None => Err(Error::new(pingora::ErrorType::ConnectNoRoute)),
        }
    }

    async fn upstream_request_filter(
        &self,
        _session: &mut Session,
        upstream_request: &mut RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<(), Box<Error>> {
        let formatted_address = _ctx
            .upstream_config
            .as_ref()
            // unwrapping because it'll be always available
            .unwrap()
            .address
            .get_formatted_address();

        upstream_request
            .insert_header("Host", formatted_address)
            .unwrap();
        Ok(())
    }
}
