use core::ffi::{CStr, c_void};

use sys::nvn::types::NVNmemoryPool;

mod builder;

pub use builder::MemoryPoolBuilder;

use crate::nvn::{BufferAddress, MemoryPoolFlags_, funcs as ffi};

#[repr(transparent)]
pub struct MemoryPool(pub NVNmemoryPool);

impl MemoryPool {
    #[inline]
    pub fn set_debug_label(&mut self, label: &CStr) {
        unsafe {
            ffi::memory_pool_set_debug_label(&mut self.0, label.as_ptr());
        }
    }

    #[inline]
    pub fn map(&self) -> *mut c_void {
        unsafe { ffi::memory_pool_map(&self.0) }
    }

    #[inline]
    pub fn flush_mapped_range(&self, offset: isize, size: usize) {
        unsafe {
            ffi::memory_pool_flush_mapped_range(&self.0, offset, size);
        }
    }

    #[inline]
    pub fn invalidate_mapped_range(&self, offset: isize, size: usize) {
        unsafe {
            ffi::memory_pool_invalidate_mapped_range(&self.0, offset, size);
        }
    }

    #[inline]
    pub fn get_buffer_address(&self) -> BufferAddress {
        unsafe { ffi::memory_pool_get_buffer_address(&self.0) }
    }

    #[inline]
    pub fn get_size(&self) -> usize {
        unsafe { ffi::memory_pool_get_size(&self.0) }
    }

    #[inline]
    pub fn get_flags(&self) -> MemoryPoolFlags_ {
        unsafe { ffi::memory_pool_get_flags(&self.0) }
    }
}

impl Drop for MemoryPool {
    fn drop(&mut self) {
        unsafe {
            ffi::memory_pool_finalize(&mut self.0);
        }
    }
}
