use std::collections::{HashMap, HashSet};

use crate::config::{backend::Backend, router_config::Gateway, upstream::Upstream};

pub struct DakiaHttpProxy {
    host_set: HashSet<String>,
    path_to_backend_map: HashMap<String, String>,
    backend_map: HashMap<String, Backend>,
    default_backend: Option<Backend>,
}

impl DakiaHttpProxy {
    fn get_host_set(gate_way: &Gateway) -> HashSet<String> {
        gate_way
            .hosts
            .iter()
            .map(|inet_address| {
                let host = &inet_address.host;
                let port = &inet_address.port;

                match port {
                    Some(port) => format!("{}:{}", host, port),
                    None => host.to_string(),
                }
            })
            .collect()
    }

    fn get_path_map(gate_way: &Gateway) -> HashMap<String, String> {
        let mut hm = HashMap::new();
        gate_way.locations.iter().for_each(|loc| {
            hm.insert(loc.path.to_string(), loc.backend.to_string());
        });
        hm
    }

    fn get_backend_map(gate_way: &Gateway) -> HashMap<String, Backend> {
        let mut hm = HashMap::new();

        gate_way.backends.iter().for_each(|backend| {
            hm.insert(backend.name.to_string(), backend.clone());
        });

        hm
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
            // TODO: avoid cloning here, use lifetime instead
            host_set: DakiaHttpProxy::get_host_set(gate_way),
            path_to_backend_map: DakiaHttpProxy::get_path_map(gate_way),
            backend_map: DakiaHttpProxy::get_backend_map(gate_way),
            default_backend: DakiaHttpProxy::get_default_backend(gate_way),
        }
    }

    pub fn get_up_stream(&self, host: String, path: String) -> Option<&Upstream> {
        if !self.is_host_exists(&host) {
            return None;
        }

        if !self.is_path_exists(&path) {
            return None;
        }

        let backend_name = self.get_path_backend(&path);

        let backend = self.get_backend(backend_name);

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
        self.host_set.contains(host)
    }

    fn is_path_exists(&self, path: &String) -> bool {
        self.path_to_backend_map.contains_key(path)
    }

    fn get_path_backend(&self, path: &String) -> &String {
        // unwrap used here because it'll be always called if path exists
        self.path_to_backend_map.get(path).unwrap()
    }

    fn get_backend(&self, backend_name: &String) -> Option<&Backend> {
        self.backend_map.get(backend_name)
    }
}

pub struct DakiaCtx {
    pub upstream_address: Option<String>,
}

impl DakiaCtx {
    pub fn new() -> DakiaCtx {
        DakiaCtx {
            upstream_address: None,
        }
    }
}
