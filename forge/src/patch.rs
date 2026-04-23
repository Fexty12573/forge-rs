use sys::patch::{Patch as SysPatch, *};

pub struct Patch {
    inner: SysPatch,
}

impl Patch {
    pub fn new(addr: u32, bytes: &[u8], enable: bool) -> Self {
        Self {
            inner: unsafe { forge_patch_create(addr, bytes.as_ptr().cast(), bytes.len() as u32, enable) },
        }
    }

    pub fn addr(&self) -> u32 {
        self.inner.addr
    }

    pub fn len(&self) -> u32 {
        self.inner.size
    }

    pub fn enabled(&self) -> bool {
        self.inner.enabled
    }

    pub fn enable(&mut self) {
        unsafe {
            forge_patch_enable(&mut self.inner);
        }
    }

    pub fn disable(&mut self) {
        unsafe {
            forge_patch_disable(&mut self.inner);
        }
    }

    pub fn patch_bytes(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.inner.patch_bytes.cast(), self.inner.size as usize) }
    }

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
