use pingora_http::ResponseHeader;
use pingora_proxy::Session;

use crate::{
    config::{source_config::GatewayConfig, DakiaConfig},
    error::{DakiaError, DakiaResult},
    shared::{common::get_dakia_version, pattern_registry::PatternRegistryType},
};

pub fn get_header<'a>(session: &'a mut Session, header_name: &'a str) -> Option<&'a str> {
    let h = session.req_header().headers.get(header_name)?;
    let conversion = h.to_str();
    match conversion {
        Ok(val) => Some(val),
        // only valid ASCII values are supported, non ASCII values in header will be treated as no value
        Err(_) => None,
    }
}

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

// a function that map DakiaError to pingora::BError
pub fn emap<T>(result: Result<T, Box<DakiaError>>) -> Result<T, pingora::BError> {
    let r = result.map_err(|e| e.to_pingora_error())?;
    Ok(r)
}
