include!(concat!(env!("OUT_DIR"), "/ascii_version.rs"));
pub fn exit() {
    std::process::exit(0);
}

pub fn get_dakia_ascii_art() -> String {
    const DAKIA_ASCII_ART: &str = "
_______
\\  ___ `'.                    .          .--.
 ' |--.\\  \\                 .'|          |__|
 | |    \\  '              .'  |          .--.
 | |     |  '     __     <    |          |  |     __
 | |     |  |  .:--.'.    |   | ____     |  |  .:--.'.
 | |     ' .' / |   \\ |   |   | \\ .'     |  | / |   \\ |
 | |___.' /'  `\" __ | |   |   |/  .      |  | `\" __ | |
/_______.'/    .'.''| |   |    /\\  \\     |__|  .'.''| |
\\_______|/    / /   | |_  |   |  \\  \\         / /   | |_ 
              \\ \\._,\\ '/  '    \\  \\  \\        \\ \\._,\\ '/
               `--'  `\"  '------'  '---'       `--'  `\"";
    DAKIA_ASCII_ART.to_string()
}

pub fn get_ascii_version() -> &'static str {
    ASCII_VERSION
}
