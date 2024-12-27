use super::Proxy;
use pingora::server::Server;
use pingora_proxy::http_proxy_service_with_name;

use crate::config::source_config::GatewayConfig;

pub fn init(server: &mut Server, gateway_config: &GatewayConfig) {
    let proxy = Proxy::build(gateway_config);
    let mut http_proxy_service =
        http_proxy_service_with_name(&server.configuration, proxy, "Dakia HTTP Proxy");

    for inet_address in &gateway_config.bind_addresses {
        let addr = inet_address.get_formatted_address();
        http_proxy_service.add_tcp(&addr);
    }

    server.add_service(http_proxy_service);
}
