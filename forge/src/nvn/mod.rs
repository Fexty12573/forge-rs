use sys::nvn::fptr::{FnDeviceGetProcAddress, NVNprocs};

use crate::os::sync::OnceLock;

mod aliases;
pub mod device;
mod funcs;
pub mod queue;
pub mod sync;
pub mod texture;

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
