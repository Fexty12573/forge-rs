# mhgu-forge-sys

Raw FFI bindings to the [`forge`](https://github.com/Fexty12573/forge) plugin
API for Monster Hunter Generations Ultimate.

This crate exposes the low-level `extern "C"` declarations and C-ABI types that
`forge` provides to plugins: hooking, patching, pattern scanning, singletons,
sockets, logging, memory, and the plugin init interface.

It is `no_std` and a building block for higher-level crates. Most plugin authors
should depend on [`mhgu-forge`](https://crates.io/crates/mhgu-forge) instead,
which wraps these bindings in a safe API.

Part of the [forge-rs](https://github.com/Fexty12573/forge-rs) workspace.
