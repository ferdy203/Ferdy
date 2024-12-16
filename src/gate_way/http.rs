use crate::config::{backend::Backend, router::Gateway, upstream::Upstream};
use std::collections::HashMap;
use wildmatch::WildMatch;

struct WildPathBackend {
    wild_path: WildMatch,
    backend: String,
}

pub struct DakiaHttpProxy {
    wild_hosts: Vec<WildMatch>,
    path_backends: Vec<WildPathBackend>,
    backend_map: HashMap<String, Backend>,
    default_backend: Option<Backend>,
}

impl DakiaHttpProxy {
    fn get_hosts(gate_way: &Gateway) -> Vec<WildMatch> {
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

    fn get_path_map(gate_way: &Gateway) -> Vec<WildPathBackend> {
        let mut path_backend_list: Vec<WildPathBackend> = vec![];

        gate_way.locations.iter().for_each(|loc| {
            let path_backend = WildPathBackend {
                wild_path: WildMatch::new(&loc.path),
                backend: loc.backend.to_string(),
            };
            path_backend_list.push(path_backend);
        });

        path_backend_list
    }

    fn get_backend_map(gate_way: &Gateway) -> HashMap<String, Backend> {
        let mut backend_map = HashMap::new();
        gate_way.backends.iter().for_each(|backend| {
            backend_map.insert(backend.name.to_string(), backend.clone());
        });
        backend_map
    }

    fn get_default_backend(gate_way: &Gateway) -> Option<Backend> {
        let default_backend_ref = gate_way.backends.iter().find(|backend| backend.default);

        match default_backend_ref {
            Some(backend_ref) => Some(backend_ref.clone()),
            None => None,
        }
    }

    pub fn build(gate_way: &Gateway) -> DakiaHttpProxy {
        DakiaHttpProxy {
            wild_hosts: DakiaHttpProxy::get_hosts(gate_way),
            path_backends: DakiaHttpProxy::get_path_map(gate_way),
            backend_map: DakiaHttpProxy::get_backend_map(gate_way),
            default_backend: DakiaHttpProxy::get_default_backend(gate_way),
        }
    }

    pub fn get_up_stream(&self, host: String, path: String) -> Option<&Upstream> {
        if !self.is_host_exists(&host) {
            return None;
        }

        let backend_name = self.get_path_backend(&path);

        let backend = match backend_name {
            None => None,
            Some(backend_name) => self.get_backend(backend_name),
        };

        match backend {
            // TODO: implement load balancer logic
            Some(backend) => backend.upstreams.get(0),
            None => match &self.default_backend {
                Some(default_backend) => default_backend.upstreams.get(0),
                None => None,
            },
        }
    }

    fn is_host_exists(&self, host: &String) -> bool {
        self.wild_hosts
            .iter()
            .any(|wild_host| wild_host.matches(host))
    }

    fn get_path_backend(&self, path: &String) -> Option<&String> {
        // unwrap used here because it'll be always called if path exists
        let path_backend = self
            .path_backends
            .iter()
            .find(|path_backend| path_backend.wild_path.matches(path));

        match path_backend {
            Some(pb) => Some(&pb.backend),
            None => None,
        }
    }

    fn get_backend(&self, backend_name: &String) -> Option<&Backend> {
        self.backend_map.get(backend_name)
    }
}

pub struct DakiaCtx {
    // TODO: use Rc or Arc to store upstream details
    pub upstream_address: Option<String>,
}

impl DakiaCtx {
    pub fn new() -> DakiaCtx {
        DakiaCtx {
            upstream_address: None,
        }
    }
}
