# mhgu-forge

Rust API for writing [`forge`](https://github.com/Fexty12573/forge) plugins for
Monster Hunter Generations Ultimate.

This is the crate you depend on when writing a plugin. It provides safe (-ish) wrappers
over the game's MT Framework types (`MtObject`, `MtDti`, `MtProperty`,
`MtFile`, …), the `SingletonManager`, function hooking, byte patching, pattern
scanning, logging, and a global allocator.

## Usage

Most users don't add this crate by hand - scaffold a plugin with
[`cargo-mhgu-forge`](https://crates.io/crates/cargo-mhgu-forge) instead:

```sh
cargo install cargo-mhgu-forge
cargo forge new my-amazing-plugin
cd my-amazing-plugin
cargo forge build --release
```

## Features

- `mt` *(default)*: MT Framework type bindings. Pulls in `allocator`.
- `allocator`: register a global allocator backed by the game's heap.
- `patterns`: pattern-scanning helpers. Pulls in `allocator`.

See the [workspace README](https://github.com/Fexty12573/forge-rs) for the full
quickstart.
