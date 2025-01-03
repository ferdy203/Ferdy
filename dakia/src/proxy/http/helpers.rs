use pingora::lb::Backend;
use pingora_http::ResponseHeader;
use pingora_proxy::Session;

use crate::{
    config::{source_config::GatewayConfig, DakiaConfig, InetAddress},
    error::{DakiaError, DakiaResult},
    qe::query::SupplierValue,
    shared::{common::get_dakia_version, pattern_registry::PatternRegistryType},
};

use super::DakiaHttpGatewayCtx;



pub fn add_dakia_header(response_header: &mut ResponseHeader) -> Result<(), Box<pingora::Error>> {
    // TODO:cotrol addition this header via flag
    let server_header = String::from("dakia/") + get_dakia_version();
    response_header.insert_header("Server", server_header)?;
    Ok(())
}

pub async fn write_response_ds(
    session: &mut Session,
    code: u16,
    keepalive: Option<u64>,
) -> Result<(), Box<pingora::Error>> {
    let mut response_header = ResponseHeader::build(code, None)?;
    add_dakia_header(&mut response_header)?;
    session.set_keepalive(keepalive);
    session
        .write_response_header(Box::new(response_header), true)
        .await?;
    Ok(())
}

fn get_gateway_config<'a>(
    dakia_config: &'a DakiaConfig,
    gateway_name: &'a str,
) -> Option<&'a GatewayConfig> {
    dakia_config
        .gateways
        .iter()
        .find(|g| g.name == gateway_name)
}

fn get_ds_addrs(dakia_config: &DakiaConfig, gateway_name: &str) -> Vec<String> {
    // safe to unwrap
    let gateway_config = get_gateway_config(dakia_config, gateway_name).unwrap();
    gateway_config
        .downstreams
        .iter()
        .map(|d| d.get_formatted_address())
        .collect()
}

pub async fn is_valid_ds_host(
    dakia_config: &DakiaConfig,
    gateway_name: &str,
    ds_host_pattern_registry: &PatternRegistryType,
    ds_host: &str,
) -> DakiaResult<bool> {
    let ds_addrs = get_ds_addrs(dakia_config, gateway_name);

    for ds_addr in ds_addrs {
        let pattern = ds_host_pattern_registry
            .get(&ds_addr)
            .await?
            .ok_or(DakiaError::create(
                crate::error::ErrorType::InternalError,
                crate::error::ErrorSource::Internal,
                Some(crate::error::ImmutStr::Owned(
                    "compiled pattern for downstream not found"
                        .to_string()
                        .into_boxed_str(),
                )),
                None,
            ))?;

        let is_matched: bool = pattern.is_match(ds_host).map_err(|e| {
            println!("{}", e);
            DakiaError::create_internal()
        })?;

        if is_matched {
            return Ok(true);
        }
    }

    Ok(false)
}

pub fn get_inet_addr_from_backend(backend: &Backend) -> InetAddress {
    let addr = backend.addr.clone().to_string();
    let parts: Vec<&str> = addr.split(":").collect();

    InetAddress {
        host: parts[0].to_owned(),
        // TODO: handle unwrap
        port: parts[1].parse().unwrap(),
    }
}

fn get_ds_method<'a>(session: &'a Session) -> DakiaResult<SupplierValue<'a>> {
    let method = session.as_downstream().req_header().method.as_str();
    Ok(SupplierValue::Str(method))
}

fn get_ds_path<'a>(session: &'a Session) -> DakiaResult<SupplierValue<'a>> {
    let path = session.as_downstream().req_header().uri.path();
    Ok(SupplierValue::Str(path))
}

fn get_ds_header<'a>(session: &'a Session, header_name: &str) -> DakiaResult<SupplierValue<'a>> {
    let header = session
        .as_downstream()
        .req_header()
        .headers
        .get(header_name);

    match header {
        Some(header_value) => match header_value.to_str() {
            Ok(hv) => return Ok(SupplierValue::Str(hv)),
            Err(e) => {
                return Err(DakiaError::because(
                    crate::error::ErrorType::InternalError,
                    format!("can not parse header value for {header_name}"),
                    e,
                ))
            }
        },
        None => Ok(SupplierValue::None),
    }
}

fn ds_part_supplier<'a, 'b>(
    path: &'a str,
    _ctx: &DakiaHttpGatewayCtx,
    session: &'b Session,
) -> DakiaResult<SupplierValue<'b>> {
    match path {
        "method" => get_ds_method(&session),
        "path" => get_ds_path(&session),
        _ if path.starts_with("header.") => {
            let path = path.replace("header.", "");
            get_ds_header(&session, path.as_str())
        }

        // TODO: add arm for query
        // TODO: add arm for cookie
        _ => Err(DakiaError::i_explain("invalid")),
    }
    // header
    // path
}

fn us_part_supplier<'a, 'b>(
    _path: &'a str,
    _ctx: &DakiaHttpGatewayCtx,
    _session: &'b Session,
) -> DakiaResult<SupplierValue<'b>> {
    todo!()
}

pub fn part_supplier<'a, 'b>(
    path: &'a str,
    ctx: &DakiaHttpGatewayCtx,
    session: &'b Session,
) -> DakiaResult<SupplierValue<'b>> {
    match path {
        // supply parts of downstream request
        _ if path.starts_with("ds.req.") => {
            let path = path.replace("ds.req.", "");
            ds_part_supplier(path.as_str(), ctx, session)
        }

        // supply parts of upstream request
        _ if path.starts_with("us.req.") => {
            let path = path.replace("us.req.", "");
            us_part_supplier(path.as_str(), ctx, session)
        }

        // error otherwise
        _ => Err(DakiaError::i_explain(format!(
            "query path \"{path}\" does not start with \"ds.req\" or \"us.req\""
        ))),
    }
}
