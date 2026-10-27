use core::ffi::c_void;

use sys::nvn::types::NVNmemoryPoolBuilder;

use crate::nvn::owned::Owned;
use crate::nvn::{MemoryPoolFlags_, device::Device, funcs as ffi, memory_pool::MemoryPool};

#[repr(transparent)]
pub struct MemoryPoolBuilder(pub NVNmemoryPoolBuilder);

impl MemoryPoolBuilder {
    pub fn new() -> Self {
        let mut b = MemoryPoolBuilder(NVNmemoryPoolBuilder::default());
        unsafe {
            ffi::memory_pool_builder_set_defaults(&mut b.0);
        }

        b
    }

    pub fn set_device(&mut self, device: &Device) -> &mut Self {
        unsafe {
            ffi::memory_pool_builder_set_device(&mut self.0, device.as_ptr());
        }
        self
    }

    pub fn set_storage(&mut self, memory: *mut c_void, size: usize) -> &mut Self {
        unsafe {
            ffi::memory_pool_builder_set_storage(&mut self.0, memory, size);
        }
        self
    }

    pub fn set_flags(&mut self, flags: MemoryPoolFlags_) -> &mut Self {
        unsafe {
            ffi::memory_pool_builder_set_flags(&mut self.0, flags);
        }
        self
    }

    pub fn get_memory(&self) -> *mut c_void {
        unsafe { ffi::memory_pool_builder_get_memory(&self.0) }
    }

    pub fn get_size(&self) -> usize {
        unsafe { ffi::memory_pool_builder_get_size(&self.0) }
    }

    pub fn get_flags(&self) -> MemoryPoolFlags_ {
        unsafe { ffi::memory_pool_builder_get_flags(&self.0) }
    }

    pub fn build(&self) -> Option<Owned<MemoryPool>> {
        Owned::<MemoryPool>::try_new(|pool| unsafe { ffi::memory_pool_initialize(pool.as_ptr(), &self.0) })
    }
}
