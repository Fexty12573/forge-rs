use sys::nvn::types::{NVNbuffer, NVNbufferBuilder};

use crate::nvn::{buffer::Buffer, device::Device, funcs as ffi, memory_pool::MemoryPool};

#[repr(transparent)]
pub struct BufferBuilder(pub NVNbufferBuilder);

impl BufferBuilder {
    pub fn new() -> Self {
        let mut b = BufferBuilder(NVNbufferBuilder::default());
        unsafe {
            ffi::buffer_builder_set_defaults(&mut b.0);
        }

        b
    }

    pub fn set_device(&mut self, device: &Device) -> &mut Self {
        unsafe {
            ffi::buffer_builder_set_device(&mut self.0, device.as_ptr());
        }
        self
    }

    pub fn set_storage(&mut self, pool: &MemoryPool, offset: isize, size: usize) -> &mut Self {
        unsafe {
            ffi::buffer_builder_set_storage(&mut self.0, (&raw const pool.0).cast_mut(), offset, size);
        }
        self
    }

    pub fn get_memory_pool(&self) -> Option<&MemoryPool> {
        unsafe { ffi::buffer_builder_get_memory_pool(&self.0).cast::<MemoryPool>().as_ref() }
    }

    pub fn get_memory_offset(&self) -> isize {
        unsafe { ffi::buffer_builder_get_memory_offset(&self.0) }
    }

    pub fn get_size(&self) -> usize {
        unsafe { ffi::buffer_builder_get_size(&self.0) }
    }

    pub fn build(&self) -> Buffer {
        let mut buffer = NVNbuffer::default();
        unsafe {
            ffi::buffer_initialize(&mut buffer, &self.0);
        }

        Buffer(buffer)
    }
}
