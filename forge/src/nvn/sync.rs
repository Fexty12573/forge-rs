use core::cell::UnsafeCell;
use core::ffi::CStr;
use core::time::Duration;

use sys::nvn::types::NVNsync;

use crate::nvn::device::Device;

use crate::nvn::owned::{Owned, impl_object};
use crate::nvn::{SyncWaitResult, funcs as ffi};

#[repr(transparent)]
pub struct Sync(UnsafeCell<NVNsync>);

impl_object!(Sync, NVNsync, ffi::sync_finalize);

impl Sync {
    #[inline]
    pub fn as_ptr(&self) -> *mut NVNsync {
        self.0.get()
    }

    pub fn new(device: &Device) -> Option<Owned<Self>> {
        Owned::<Self>::try_new(|sync| unsafe { ffi::sync_initialize(sync.as_ptr(), device.as_ptr()) })
    }

    #[inline]
    pub fn set_debug_label(&mut self, label: &CStr) {
        unsafe {
            ffi::sync_set_debug_label(self.as_ptr(), label.as_ptr());
        }
    }

    #[inline]
    pub fn wait(&self, timeout: Duration) -> SyncWaitResult {
        unsafe { ffi::sync_wait(self.as_ptr(), u64::try_from(timeout.as_nanos()).unwrap_or(u64::MAX)) }
    }
}
