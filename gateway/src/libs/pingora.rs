use pingora::Error;
use pingora_proxy::Session;

pub fn get_header_value(
    _session: &mut Session,
    header_name: String,
) -> Result<Option<&str>, Box<Error>> {
    let host_header_value = _session.req_header().headers.get(header_name);

    let host_header_str_result = match host_header_value {
        Some(host_header_value) => match host_header_value.to_str() {
            Ok(header_value) => Ok(header_value),
            Err(e) => Err(e),
        },
        None => Ok(""),
    };

    let header_value = host_header_str_result.unwrap_or_else(|_| "");
    if header_value.is_empty() || header_value.len() == 0 {
        Ok(None)
    } else {
        Ok(Some(header_value))
    }
}
