use core::ffi::CStr;
use core::time::Duration;

use sys::nvn::types::NVNsync;

use crate::nvn::device::Device;

use crate::nvn::{SyncWaitResult, funcs as ffi};

pub struct Sync(pub NVNsync);

impl Sync {
    pub fn new(device: &Device) -> Self {
        let mut sync = NVNsync::default();
        unsafe {
            ffi::sync_initialize(&mut sync, device.as_ptr());
        }

        Self(sync)
    }

    #[inline]
    pub fn set_debug_label(&mut self, label: &CStr) {
        unsafe {
            ffi::sync_set_debug_label(&mut self.0, label.as_ptr());
        }
    }

    #[inline]
    pub fn wait(&self, timeout: Duration) -> SyncWaitResult {
        unsafe { ffi::sync_wait(&self.0, timeout.as_nanos() as u64) }
    }
}

impl Drop for Sync {
    fn drop(&mut self) {
        unsafe {
            ffi::sync_finalize(&mut self.0);
        }
    }
}
