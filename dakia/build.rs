use figlet_rs::FIGfont;
use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    // Get the package version at build time
    let version = env!("CARGO_PKG_VERSION");

    // Generate the ASCII art using figlet
    let font = FIGfont::standard().unwrap();

    let ascii_version = font.convert(version).unwrap();

    // Embed the ASCII art directly into the Rust code using `println!`
    println!("cargo:rerun-if-changed=build.rs");

    // Output the ASCII art to a file (build.rs will use this in the main code)
    let out_dir = env::var("OUT_DIR").unwrap();
    let ascii_version_path = format!("{}/ascii_version.rs", out_dir);

    let mut file = File::create(ascii_version_path).unwrap();
    file.write_all(format!("pub const ASCII_VERSION: &str = r#\"{}\"#;", ascii_version).as_bytes())
        .unwrap();

    // write dakia ascii art
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

    let dakia_ascii_art_path = format!("{}/dakia_ascii_art.rs", out_dir);

    let mut file = File::create(dakia_ascii_art_path).unwrap();
    file.write_all(
        format!(
            "pub const DAKIA_ASCII_ART: &str = r#\"{}\"#;",
            DAKIA_ASCII_ART
        )
        .as_bytes(),
    )
    .unwrap();
}
