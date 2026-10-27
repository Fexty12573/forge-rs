use sys::nvn::fptr::{FnDeviceGetProcAddress, NVNprocs};

use crate::os::sync::OnceLock;

mod aliases;
pub mod buffer;
pub mod command_buffer;
pub mod device;
pub mod event;
mod funcs;
pub mod memory_pool;
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
pub use texture::{ImageHandle, TextureHandle};

#[unsafe(no_mangle)]
extern "C" fn forge_onNvnInit(get_proc_addr: Option<FnDeviceGetProcAddress>) {
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
    NVN_PROCS.get().expect("NVN has not been initialized yet")
}
