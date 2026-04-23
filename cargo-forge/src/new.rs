use crate::embedded;
use anyhow::{Context, Result, bail};
use std::path::Path;

const FORGE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn new(name: &str, dest: &Path, git: bool) -> Result<()> {
    let project_dir = dest.join(name);
    if project_dir.exists() {
        bail!("Directory '{}' already exists", project_dir.display());
    }

    write_file(&project_dir.join("Cargo.toml"), &cargo_toml(name))?;
    write_file(&project_dir.join("src").join("lib.rs"), &plugin_lib_rs())?;
    write_file(&project_dir.join("build.rs"), BUILD_RS)?;
    write_file(&project_dir.join(".cargo").join("config.toml"), &cargo_config())?;
    write_file(&project_dir.join("link").join("switch32.ld"), embedded::SWITCH32_LD)?;
    write_file(&project_dir.join("link").join("exported.txt"), embedded::EXPORTED_TXT)?;
    write_file(&project_dir.join("rust-toolchain.toml"), embedded::RUST_TOOLCHAIN)?;
    write_file(
        &project_dir.join(format!("{}.json", embedded::TARGET_NAME)),
        embedded::TARGET_JSON,
    )?;

    if git {
        write_file(&project_dir.join(".gitignore"), GITIGNORE)?;
    }

    println!("Created forge plugin '{name}' at {}", project_dir.display());
    println!("Build with: cd {name} && cargo forge build");

    Ok(())
}

fn write_file(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?
    }

    std::fs::write(path, content).with_context(|| format!("writing {}", path.display()))
}

fn cargo_toml(name: &str) -> String {
    format!(
        r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
mhgu-forge = "{FORGE_VERSION}"
"#
    )
}

fn plugin_lib_rs() -> String {
    format!(
        r#"#![no_std]

#[forge::entry]
fn main() {{}}
    "#
    )
}

fn cargo_config() -> String {
    format!(
        r#"[build]
target = "{}.json"

[unstable]
build-std = ["core", "alloc"]
build-std-features = ["compiler-builtins-mem"]
json-target-spec = true
"#,
        embedded::TARGET_NAME
    )
}

const BUILD_RS: &str = r#"
fn main() {
    let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-arg=-T{dir}/link/switch32.ld");
    println!("cargo:rustc-link-arg=-Wl,--build-id=sha1");
    println!("cargo:rerun-if-changed=link/switch32.ld");
}
"#;

const GITIGNORE: &str = r#"
target/
*.nro
"#;
