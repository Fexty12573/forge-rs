# cargo-mhgu-forge

A `cargo` subcommand for initializing and building
[`forge`](https://github.com/Fexty12573/forge) plugins for Monster Hunter
Generations Ultimate.

## Install

```sh
cargo install cargo-mhgu-forge
```

## Usage

Create a new plugin project (pass `-g` to also add a `.gitignore`):

```sh
cargo forge new my-amazing-plugin
```

Build it - this compiles the plugin and converts the resulting ELF into a
loadable `.nro` (no external `elf2nro` tool required):

```sh
cd my-amazing-plugin
cargo forge build --release
```

The output `my-amazing-plugin.nro` goes in `romfs/nativeNX/plugins` to be loaded
by `forge`.

Part of the [forge-rs](https://github.com/Fexty12573/forge-rs) workspace.
