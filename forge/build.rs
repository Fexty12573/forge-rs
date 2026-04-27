use std::{env, fs, path::Path};

fn main() {
    let major: u16 = env::var("CARGO_PKG_VERSION_MAJOR").unwrap().parse().unwrap();
    let minor: u16 = env::var("CARGO_PKG_VERSION_MINOR").unwrap().parse().unwrap();
    let patch: u16 = env::var("CARGO_PKG_VERSION_PATCH").unwrap().parse().unwrap();

    let out = env::var("OUT_DIR").unwrap();
    fs::write(
        Path::new(&out).join("version.rs"),
        format!(
            "pub const REQUIRED_VERSION: crate::ForgeVersion = crate::ForgeVersion \
             {{ major: {major}, minor: {minor}, patch: {patch} }};\n"
        ),
    )
    .unwrap();

    println!("cargo:rerun-if-changed=Cargo.toml");
}
