pub mod color;
pub mod context;
pub mod drag;
pub mod draw;
pub mod font;
pub mod input;
pub mod item;
pub mod layout;
pub mod menu;
pub mod plot;
pub mod popup;
pub mod slider;
pub mod stack;
pub mod table;
pub mod tooltip;
pub mod tree;
pub mod widgets;
pub mod window;

pub use color::*;
pub use context::*;
pub use drag::*;
pub use draw::*;
pub use font::*;
pub use input::*;
pub use item::*;
pub use layout::*;
pub use menu::*;
pub use plot::*;
pub use popup::*;
pub use slider::*;
pub use stack::*;
pub use table::*;
pub use tooltip::*;
pub use tree::*;
pub use widgets::*;
pub use window::*;

#[cfg(feature = "nvn")]
use crate::nvn::{
    TextureHandle,
    texture::{self, Texture, TextureView},
};

pub enum Id {
    Int(u32),
    Str(&'static core::ffi::CStr),
}

/// Registers a texture for use in ImGui. Returns a handle that can be used to reference the texture in ImGui calls.
/// A `None` view indicates that the entire texture should be used.
///
/// # Safety
/// The texture and view must remain valid for the lifetime of the handle. The handle must be
/// unregistered with [`unregister_texture`] before the texture and view are destroyed.
#[cfg(feature = "nvn")]
pub fn register_texture(texture: &Texture, view: Option<&TextureView>) -> TextureHandle {
    unsafe { sys::graphics::forge_graphics_registerTexture(texture.as_ptr(), texture::view_ptr(view)) }
}

/// Unregisters a texture handle previously registered with [`register_texture`].
///
/// # Safety
/// The handle must have been previously registered with [`register_texture`].
#[cfg(feature = "nvn")]
pub fn unregister_texture(handle: TextureHandle) {
    unsafe {
        sys::graphics::forge_graphics_unregisterTexture(handle);
    }
}
