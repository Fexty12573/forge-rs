use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};

mod embedded;
mod fbuild;
mod init;

#[derive(Parser)]
#[command(name = "cargo")]
enum Cargo {
    #[command(name = "forge", about = "Forge plugin toolchain")]
    Forge(ForgeArgs),
}

#[derive(Args)]
struct ForgeArgs {
    #[command(subcommand)]
    cmd: ForgeCommand,
}

#[derive(Subcommand)]
enum ForgeCommand {
    // Create a new forge plugin project
    Init {
        name: String,

        #[arg(long, default_value = ".")]
        path: PathBuf,
    },

    // Build a forge plugin and convert the resulting ELF to an NRO
    Build {
        #[arg(long)]
        release: bool,
    },
}

fn main() -> Result<()> {
    let Cargo::Forge(args) = Cargo::parse();

    match args.cmd {
        ForgeCommand::Init { name, path } => init::init(&name, &path),
        ForgeCommand::Build { release } => fbuild::build(release),
    }
}
