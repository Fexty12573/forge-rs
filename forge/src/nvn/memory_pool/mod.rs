use core::cell::UnsafeCell;
use core::ffi::{CStr, c_void};

use sys::nvn::types::NVNmemoryPool;

mod builder;

pub use builder::MemoryPoolBuilder;

use crate::nvn::owned::impl_object;
use crate::nvn::{BufferAddress, MemoryPoolFlags_, funcs as ffi};

#[repr(transparent)]
pub struct MemoryPool(UnsafeCell<NVNmemoryPool>);

impl_object!(MemoryPool, NVNmemoryPool, ffi::memory_pool_finalize);

impl MemoryPool {
    #[inline]
    pub fn as_ptr(&self) -> *mut NVNmemoryPool {
        self.0.get()
    }

    #[inline]
    pub fn set_debug_label(&mut self, label: &CStr) {
        unsafe {
            ffi::memory_pool_set_debug_label(self.as_ptr(), label.as_ptr());
        }
    }

    #[inline]
    pub fn map(&self) -> *mut c_void {
        unsafe { ffi::memory_pool_map(self.as_ptr()) }
    }

    #[inline]
    pub fn flush_mapped_range(&self, offset: isize, size: usize) {
        unsafe {
            ffi::memory_pool_flush_mapped_range(self.as_ptr(), offset, size);
        }
    }

    #[inline]
    pub fn invalidate_mapped_range(&self, offset: isize, size: usize) {
        unsafe {
            ffi::memory_pool_invalidate_mapped_range(self.as_ptr(), offset, size);
        }
    }

    #[inline]
    pub fn get_buffer_address(&self) -> BufferAddress {
        unsafe { ffi::memory_pool_get_buffer_address(self.as_ptr()) }
    }

    #[inline]
    pub fn get_size(&self) -> usize {
        unsafe { ffi::memory_pool_get_size(self.as_ptr()) }
    }

    #[inline]
    pub fn get_flags(&self) -> MemoryPoolFlags_ {
        unsafe { ffi::memory_pool_get_flags(self.as_ptr()) }
    }
}
