use core::ffi::c_void;

use sys::nvn::types::NVNqueueBuilder;

#[repr(transparent)]
pub struct QueueBuilder(pub NVNqueueBuilder);

use crate::nvn::owned::Owned;
use crate::nvn::{
    QueueBuilderFlags_,
    device::Device,
    funcs::{self as ffi},
    queue::Queue,
};

impl QueueBuilder {
    pub fn new() -> Self {
        let mut b = QueueBuilder(NVNqueueBuilder::default());
        unsafe {
            ffi::queue_builder_set_defaults(&mut b.0);
        }

        b
    }

    pub fn set_device(&mut self, device: &Device) -> &mut Self {
        unsafe {
            ffi::queue_builder_set_device(&mut self.0, device.as_ptr());
        }
        self
    }

    pub fn set_flags(&mut self, flags: QueueBuilderFlags_) -> &mut Self {
        unsafe {
            ffi::queue_builder_set_flags(&mut self.0, flags);
        }
        self
    }

    pub fn set_command_mem_size(&mut self, size: usize) -> &mut Self {
        unsafe {
            ffi::queue_builder_set_command_memory_size(&mut self.0, size);
        }
        self
    }

    pub fn set_control_mem_size(&mut self, size: usize) -> &mut Self {
        unsafe {
            ffi::queue_builder_set_control_memory_size(&mut self.0, size);
        }
        self
    }

    pub fn set_compute_mem_size(&mut self, size: usize) -> &mut Self {
        unsafe {
            ffi::queue_builder_set_compute_memory_size(&mut self.0, size);
        }
        self
    }

    pub fn set_command_flush_threshold(&mut self, threshold: usize) -> &mut Self {
        unsafe {
            ffi::queue_builder_set_command_flush_threshold(&mut self.0, threshold);
        }
        self
    }

    pub fn set_queue_mem(&mut self, memory: *mut c_void, size: usize) -> &mut Self {
        unsafe {
            ffi::queue_builder_set_queue_memory(&mut self.0, memory, size);
        }
        self
    }

    pub fn get_device(&self) -> Option<&Device> {
        unsafe { ffi::queue_builder_get_device(&self.0).cast::<Device>().as_ref() }
    }

    pub fn get_flags(&self) -> QueueBuilderFlags_ {
        unsafe { ffi::queue_builder_get_flags(&self.0) }
    }

    pub fn get_command_mem_size(&self) -> usize {
        unsafe { ffi::queue_builder_get_command_memory_size(&self.0) }
    }

    pub fn get_queue_mem_size(&self) -> usize {
        unsafe { ffi::queue_builder_get_queue_memory_size(&self.0) }
    }

    pub fn build(&self) -> Option<Owned<Queue>> {
        Owned::<Queue>::try_new(|queue| unsafe { ffi::queue_initialize(queue.as_ptr(), &self.0) })
    }
}
