use anyhow::{Context, Result, bail};
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::embedded::TARGET_NAME;

#[derive(Deserialize)]
struct CargoManifest {
    package: Package,
}

#[derive(Deserialize)]
struct Package {
    name: String,
}

pub fn build(release: bool) -> Result<()> {
    let manifest_dir = std::env::current_dir().context("cannot determine current directory")?;
    let package_name = read_package_name(&manifest_dir)?;

    let mut cmd = Command::new("cargo");
    cmd.arg("-Zjson-target-spec");
    cmd.arg("build");
    if release {
        cmd.arg("--release");
    }

    // Check if devKitARM is available. If so, use it for linking.
    let linker_key = format!("CARGO_TARGET_{}_LINKER", TARGET_NAME.replace('-', "_").to_uppercase());
    if let Ok(devkitarm) = std::env::var("DEVKITARM") {
        let linker = PathBuf::from(devkitarm).join("bin").join("arm-none-eabi-gcc");
        if linker.exists() {
            cmd.env(&linker_key, linker);
        }
    } else if let Ok(devkitpro) = std::env::var("DEVKITPRO") {
        let linker = PathBuf::from(devkitpro)
            .join("devkitARM")
            .join("bin")
            .join("arm-none-eabi-gcc");
        if linker.exists() {
            cmd.env(&linker_key, linker);
        }
    }

    let status = cmd.status().context("failed to run cargo build")?;
    if !status.success() {
        bail!("cargo build failed");
    }

    let profile = if release { "release" } else { "debug" };
    let target_dir = manifest_dir.join("target").join(TARGET_NAME).join(profile);

    let elf_path =
        find_elf(&target_dir, &package_name).with_context(|| format!("could not find built ELF in {}", target_dir.display()))?;

    println!("Built ELF: {}", elf_path.display());

    let elf2nro = locate_elf2nro()?;
    let nro_path = manifest_dir.join(format!("{package_name}.nro"));

    let status = Command::new(&elf2nro)
        .arg(&elf_path)
        .arg(&nro_path)
        .status()
        .with_context(|| format!("failed to run {}", elf2nro.display()))?;

    if !status.success() {
        bail!("elf2nro32 failed");
    }

    println!("Output NRO: {}", nro_path.display());
    Ok(())
}

fn read_package_name(dir: &Path) -> Result<String> {
    let toml_bytes = std::fs::read(dir.join("Cargo.toml")).context("cannot read Cargo.toml")?;
    let manifest: CargoManifest = toml::from_str(&String::from_utf8_lossy(&toml_bytes)).context("cannot parse Cargo.toml")?;
    Ok(manifest.package.name.replace('-', "_"))
}

fn find_elf(dir: &Path, crate_name: &str) -> Option<PathBuf> {
    let candidates = [format!("lib{crate_name}.so"), format!("lib{crate_name}")];

    for name in &candidates {
        let p = dir.join(name);
        if p.exists() {
            return Some(p);
        }
    }

    None
}

fn locate_elf2nro() -> Result<PathBuf> {
    // Prefer explicit override
    if let Ok(path) = std::env::var("ELF2NRO32") {
        return Ok(PathBuf::from(path));
    }

    // Standard devkitPRO location
    if let Ok(devkitpro) = std::env::var("DEVKITPRO") {
        let p = PathBuf::from(devkitpro).join("tools").join("bin").join("elf2nro32");
        if p.exists() {
            return Ok(p);
        }
    }

    // Fall back to PATH
    Ok(PathBuf::from("elf2nro32"))
}
