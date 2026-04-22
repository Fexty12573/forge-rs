use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};

mod embedded;
mod fbuild;
mod new;

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
    New {
        name: String,

        #[arg(long, default_value = ".")]
        path: PathBuf,

        #[arg(short, long, action, help = "Add a .gitignore")]
        git: bool,
    },

    // Build a forge plugin and convert the resulting ELF to an NRO
    Build {
        #[arg(short, long, help = "Build in release mode")]
        release: bool,
    },
}

fn main() -> Result<()> {
    let Cargo::Forge(args) = Cargo::parse();

    match args.cmd {
        ForgeCommand::New { name, path, git } => new::new(&name, &path, git),
        ForgeCommand::Build { release } => fbuild::build(release),
    }
}
