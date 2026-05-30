# mhgu-forge-macros

Procedural macros for [`forge`](https://github.com/Fexty12573/forge) plugins.

Provides the derives and attribute macros used by
[`mhgu-forge`](https://crates.io/crates/mhgu-forge), such as `#[derive(Object)]`
/ `#[derive(HasVtable)]` / `#[derive(CacheDti)]`, the `#[pure_virtual(n)]`
vtable-slot attribute, and hook helpers.

You don't usually depend on this crate directly, it is re-exported through
`mhgu-forge`.

Part of the [forge-rs](https://github.com/Fexty12573/forge-rs) workspace.
