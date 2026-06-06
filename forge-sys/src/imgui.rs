//! Raw FFI bindings to cimgui (Dear ImGui 1.92.8).
//!
//! These are the unsafe, 1:1 cimgui declarations. The symbols themselves are
//! resolved at runtime by the loader, which hosts the actual Dear ImGui (with
//! cimgui compiled in) — nothing is linked or compiled from C here. Safe/manual
//! wrappers are built on top of these.
//!
//! `imgui_bindings.rs` is committed (not generated at build time) so consumers
//! only need a Rust toolchain. Regenerate it after a cimgui version bump with:
//!
//!     cargo run --manifest-path tools/imgui-bindgen/Cargo.toml
#![allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    dead_code,
    unnecessary_transmutes,
    clippy::all
)]

include!("imgui_bindings.rs");
