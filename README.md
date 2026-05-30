# forge-rs

This repo hosts a set of Rust Crates for the [`forge`](https://github.com/Fexty12573/forge) plugin loader for Monster Hunter Generations Ultimate.

## Quickstart

### Prerequisites

To load plugins, you need to have the [`forge`](https://github.com/Fexty12573/forge/releases) plugin loader installed.

### Creating a Plugin

1\. Install `cargo-mhgu-forge`

```sh
cargo install cargo-mhgu-forge
```

2\. Create a new Plugin

```sh
cargo forge new my-amazing-plugin
```

Optionally pass `-g` to add a `.gitignore` as well.

3\. Build your Plugin

```sh
cd my-amazing-plugin
cargo forge build --release
```

Now you should find a `.nro` file in your Plugins root directory, e.g. `my-amazing-plugin.nro`. This file now needs to be placed into `romfs/nativeNX/plugins` and it will be loaded by `forge`.
