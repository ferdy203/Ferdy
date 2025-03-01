include!(concat!(env!("OUT_DIR"), "/ascii_version.rs"));
include!(concat!(env!("OUT_DIR"), "/dakia_ascii_art.rs"));

pub fn exit() {
    std::process::exit(0);
}

pub fn get_dakia_ascii_art() -> String {
    DAKIA_ASCII_ART.to_string() + "\n\n" + get_ascii_version()
}

pub fn get_ascii_version() -> &'static str {
    ASCII_VERSION
}

pub fn _get_dakia_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
