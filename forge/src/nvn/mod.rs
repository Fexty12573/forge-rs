use sys::{
    graphics::forge_graphics_getSwapChainTextures,
    nvn::fptr::{FnDeviceGetProcAddress, NVNprocs},
};

use crate::os::sync::OnceLock;

mod aliases;
pub mod buffer;
pub mod command_buffer;
pub mod device;
pub mod event;
mod funcs;
pub mod memory_pool;
pub mod owned;
pub mod program;
pub mod queue;
pub mod sampler;
pub mod sampler_pool;
pub mod state;
pub mod sync;
pub mod texture;
pub mod texture_pool;
pub mod window;

pub use aliases::*;
pub use owned::{Object, Owned};
pub use texture::{ImageHandle, TextureHandle};

#[doc(hidden)]
pub fn __load_procs(get_proc_addr: Option<FnDeviceGetProcAddress>) {
    if let Some(func) = get_proc_addr {
        NVN_PROCS.get_or_init(|| unsafe { NVNprocs::load(core::ptr::null(), func) });
    }
}

static NVN_PROCS: OnceLock<NVNprocs> = OnceLock::new();

/// Returns the loaded NVN function table.
///
/// # Panics
/// Panics if called before the loader has initialized NVN.
#[inline]
pub fn procs() -> &'static NVNprocs {
    NVN_PROCS
        .get()
        .expect("NVN has not been initialized yet. Make sure you have a function marked #[nvn_init]")
}

/// Calls f with the current swap-chain render targets.
pub fn with_render_targets(f: impl FnOnce(&[&texture::Texture])) {
    let mut count = 0;
    let textures = unsafe {
        let ptrs = forge_graphics_getSwapChainTextures(&mut count);
        if ptrs.is_null() || count == 0 {
            &[]
        } else {
            ::core::slice::from_raw_parts(ptrs.cast(), count)
        }
    };

    f(textures);
}
