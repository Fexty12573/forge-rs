use sys::patch::{Patch as SysPatch, *};

/// A memory patch that can be enabled or disabled at runtime.
/// When enabled, the patch will overwrite the target bytes with the patch bytes.
/// When disabled, the original bytes will be restored.
///
/// The patch will be automatically disabled and cleaned up when dropped.
pub struct Patch {
    inner: SysPatch,
}

impl Patch {
    /// Creates a new patch at the given address with the given patch bytes.
    /// The patch is enabled if `enable` is true, otherwise it is created in a disabled state.
    pub fn new(addr: u32, bytes: &[u8], enable: bool) -> Self {
        Self {
            inner: unsafe { forge_patch_create(addr, bytes.as_ptr().cast(), bytes.len() as u32, enable) },
        }
    }

    /// Gets the address that this patch targets.
    pub fn addr(&self) -> u32 {
        self.inner.addr
    }

    /// Gets the length of this patch in bytes.
    pub fn len(&self) -> u32 {
        self.inner.size
    }

    /// Returns `true` if this patch is currently enabled.
    pub fn enabled(&self) -> bool {
        self.inner.enabled
    }

    /// Enables this patch, applying the patch bytes to the target address.
    pub fn enable(&mut self) {
        unsafe {
            forge_patch_enable(&mut self.inner);
        }
    }

    /// Disables this patch, restoring the original bytes at the target address.
    pub fn disable(&mut self) {
        unsafe {
            forge_patch_disable(&mut self.inner);
        }
    }

    /// Returns a byte slice of the patch bytes that will be written when this patch is enabled.
    pub fn patch_bytes(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.inner.patch_bytes.cast(), self.inner.size as usize) }
    }

    /// Returns a byte slice of the original bytes that are overwritten when this patch is enabled.
    pub fn original_bytes(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.inner.original_bytes.cast(), self.inner.size as usize) }
    }
}

impl core::ops::Drop for Patch {
    fn drop(&mut self) {
        unsafe {
            forge_patch_destroy(&mut self.inner);
        }
    }
}
