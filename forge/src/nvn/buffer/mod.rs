use core::cell::UnsafeCell;
use core::ffi::{CStr, c_void};

use sys::nvn::types::NVNbuffer;

mod builder;

pub use builder::BufferBuilder;

use crate::nvn::owned::impl_object;
use crate::nvn::{BufferAddress, funcs as ffi, memory_pool::MemoryPool};

#[repr(transparent)]
pub struct Buffer(UnsafeCell<NVNbuffer>);

impl_object!(Buffer, NVNbuffer, ffi::buffer_finalize);

impl Buffer {
    #[inline]
    pub fn as_ptr(&self) -> *mut NVNbuffer {
        self.0.get()
    }

    #[inline]
    pub fn set_debug_label(&mut self, label: &CStr) {
        unsafe {
            ffi::buffer_set_debug_label(self.as_ptr(), label.as_ptr());
        }
    }

    #[inline]
    pub fn map(&self) -> *mut c_void {
        unsafe { ffi::buffer_map(self.as_ptr()) }
    }

    #[inline]
    pub fn get_address(&self) -> BufferAddress {
        unsafe { ffi::buffer_get_address(self.as_ptr()) }
    }

    #[inline]
    pub fn flush_mapped_range(&self, offset: isize, size: usize) {
        unsafe {
            ffi::buffer_flush_mapped_range(self.as_ptr(), offset, size);
        }
    }

    #[inline]
    pub fn invalidate_mapped_range(&self, offset: isize, size: usize) {
        unsafe {
            ffi::buffer_invalidate_mapped_range(self.as_ptr(), offset, size);
        }
    }

    #[inline]
    pub fn get_memory_pool(&self) -> Option<&MemoryPool> {
        unsafe { ffi::buffer_get_memory_pool(self.as_ptr()).cast::<MemoryPool>().as_ref() }
    }

    #[inline]
    pub fn get_memory_offset(&self) -> isize {
        unsafe { ffi::buffer_get_memory_offset(self.as_ptr()) }
    }

    #[inline]
    pub fn get_size(&self) -> usize {
        unsafe { ffi::buffer_get_size(self.as_ptr()) }
    }

    #[inline]
    pub fn get_debug_id(&self) -> u64 {
        unsafe { ffi::buffer_get_debug_id(self.as_ptr()) }
    }
}
