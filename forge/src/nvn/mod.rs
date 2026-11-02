use core::{
    ptr,
    sync::atomic::{AtomicBool, AtomicPtr, Ordering},
};

use sys::{
    graphics::forge_graphics_getSwapChainTextures,
    nvn::{
        fptr::{FnDeviceGetProcAddress, NVNprocs},
        types::{NVNdevice, NVNqueue},
    },
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

#[unsafe(no_mangle)]
extern "C" fn forge_onNvnInit(get_proc_addr: Option<FnDeviceGetProcAddress>, device: *mut NVNdevice, queue: *mut NVNqueue) {
    if let Some(func) = get_proc_addr {
        NVN_PROCS.get_or_init(|| unsafe { NVNprocs::load(core::ptr::null(), func) });
    }

    DEVICE.store(device, Ordering::Release);
    QUEUE.store(queue, Ordering::Release);
}

static NVN_PROCS: OnceLock<NVNprocs> = OnceLock::new();

static DEVICE: AtomicPtr<NVNdevice> = AtomicPtr::new(ptr::null_mut());
static DEVICE_BORROWED: AtomicBool = AtomicBool::new(false);

static QUEUE: AtomicPtr<NVNqueue> = AtomicPtr::new(ptr::null_mut());
static QUEUE_BORROWED: AtomicBool = AtomicBool::new(false);

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

/// Runs `f` with the current swap-chain render targets.
pub fn with_render_targets<R>(f: impl FnOnce(&[&texture::Texture]) -> R) -> R {
    let mut count = 0;
    let textures = unsafe {
        let ptrs = forge_graphics_getSwapChainTextures(&mut count);
        if ptrs.is_null() || count == 0 {
            &[]
        } else {
            ::core::slice::from_raw_parts(ptrs.cast(), count)
        }
    };

    f(textures)
}

/// Runs `f` with exclusive access to the game's device.
///
/// # Panics
/// Panics if NVN isn't initialized yet, or if the device is already borrowed.
pub fn with_device<R>(f: impl FnOnce(&mut device::Device) -> R) -> R {
    let ptr = load(&DEVICE);
    let _b = Borrow::new(&DEVICE_BORROWED, "Device");
    f(unsafe { device::Device::from_ptr_mut(ptr) })
}

/// Runs `f` with exclusive access to the game's queue.
///
/// # Panics
/// Panics if NVN isn't initialized yet, or if the queue is already borrowed.
pub fn with_queue<R>(f: impl FnOnce(&mut queue::Queue) -> R) -> R {
    let ptr = load(&QUEUE);
    let _b = Borrow::new(&QUEUE_BORROWED, "Queue");
    f(unsafe { queue::Queue::from_ptr_mut(ptr) })
}

/// Runs `f` with exclusive access to the game's device, queue and render targets.
///
/// # Panics
/// Panics if NVN isn't initialized yet, or if any of the above are already borrowed.
pub fn with<R>(f: impl FnOnce(&mut device::Device, &mut queue::Queue, &[&texture::Texture]) -> R) -> R {
    let (d, q) = (load(&DEVICE), load(&QUEUE));
    let _bd = Borrow::new(&DEVICE_BORROWED, "Device");
    let _bq = Borrow::new(&QUEUE_BORROWED, "Queue");

    with_render_targets(|rts| {
        f(
            unsafe { device::Device::from_ptr_mut(d) },
            unsafe { queue::Queue::from_ptr_mut(q) },
            rts,
        )
    })
}

struct Borrow(&'static AtomicBool);

impl Borrow {
    fn new(flag: &'static AtomicBool, what: &str) -> Self {
        assert!(!flag.swap(true, Ordering::Acquire), "{what} is already borrowed");
        Self(flag)
    }
}

impl Drop for Borrow {
    fn drop(&mut self) {
        self.0.store(false, Ordering::Release);
    }
}

fn load<T>(ptr: &AtomicPtr<T>) -> *mut T {
    let p = ptr.load(Ordering::Acquire);
    assert!(!p.is_null(), "NVN has not been initialized yet");
    p
}
