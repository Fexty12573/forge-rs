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

impl ImVec2 {
    pub fn new(x: f32, y: f32) -> Self {
        ImVec2 { x, y }
    }
}

impl ImVec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}

impl From<(f32, f32)> for ImVec2 {
    fn from(value: (f32, f32)) -> Self {
        Self { x: value.0, y: value.1 }
    }
}

impl From<(f32, f32, f32, f32)> for ImVec4 {
    fn from(value: (f32, f32, f32, f32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.2,
            w: value.3,
        }
    }
}

impl Default for ImVec2 {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl Default for ImVec4 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }
}
