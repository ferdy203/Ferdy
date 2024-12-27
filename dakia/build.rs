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
    let path = format!("{}/ascii_version.rs", out_dir);

    let mut file = File::create(path).unwrap();
    file.write_all(format!("pub const ASCII_VERSION: &str = r#\"{}\"#;", ascii_version).as_bytes())
        .unwrap();
}
