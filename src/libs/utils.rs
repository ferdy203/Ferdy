pub fn get_or_default<T>(item: Option<T>, default: T) -> T
where
    T: Clone,
{
    item.unwrap_or_else(|| default.clone())
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
