use core::ffi::{CStr, c_void};

use sys::nvn::types::NVNbuffer;

mod builder;

pub use builder::BufferBuilder;

use crate::nvn::{BufferAddress, funcs as ffi, memory_pool::MemoryPool};

#[repr(transparent)]
pub struct Buffer(pub NVNbuffer);

impl Buffer {
    #[inline]
    pub fn set_debug_label(&mut self, label: &CStr) {
        unsafe {
            ffi::buffer_set_debug_label(&mut self.0, label.as_ptr());
        }
    }

    #[inline]
    pub fn map(&self) -> *mut c_void {
        unsafe { ffi::buffer_map(&self.0) }
    }

    #[inline]
    pub fn get_address(&self) -> BufferAddress {
        unsafe { ffi::buffer_get_address(&self.0) }
    }

    #[inline]
    pub fn flush_mapped_range(&self, offset: isize, size: usize) {
        unsafe {
            ffi::buffer_flush_mapped_range(&self.0, offset, size);
        }
    }

    #[inline]
    pub fn invalidate_mapped_range(&self, offset: isize, size: usize) {
        unsafe {
            ffi::buffer_invalidate_mapped_range(&self.0, offset, size);
        }
    }

    #[inline]
    pub fn get_memory_pool(&self) -> Option<&MemoryPool> {
        unsafe { ffi::buffer_get_memory_pool(&self.0).cast::<MemoryPool>().as_ref() }
    }

    #[inline]
    pub fn get_memory_offset(&self) -> isize {
        unsafe { ffi::buffer_get_memory_offset(&self.0) }
    }

    #[inline]
    pub fn get_size(&self) -> usize {
        unsafe { ffi::buffer_get_size(&self.0) }
    }

    #[inline]
    pub fn get_debug_id(&self) -> u64 {
        unsafe { ffi::buffer_get_debug_id(&self.0) }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            ffi::buffer_finalize(&mut self.0);
        }
    }
}
