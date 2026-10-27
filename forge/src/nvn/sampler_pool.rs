use core::ffi::CStr;

use sys::nvn::types::NVNsamplerPool;

use crate::nvn::{
    funcs as ffi,
    memory_pool::MemoryPool,
    sampler::{Sampler, SamplerBuilder},
};

#[repr(transparent)]
pub struct SamplerPool(pub NVNsamplerPool);

impl SamplerPool {
    pub fn new(memory_pool: &MemoryPool, offset: isize, num_descriptors: i32) -> Self {
        let mut pool = NVNsamplerPool::default();
        unsafe {
            ffi::sampler_pool_initialize(&mut pool, &memory_pool.0, offset, num_descriptors);
        }

        Self(pool)
    }

    #[inline]
    pub fn set_debug_label(&mut self, label: &CStr) {
        unsafe {
            ffi::sampler_pool_set_debug_label(&mut self.0, label.as_ptr());
        }
    }

    #[inline]
    pub fn register_sampler(&self, id: i32, sampler: &Sampler) {
        unsafe {
            ffi::sampler_pool_register_sampler(&self.0, id, &sampler.0);
        }
    }

    #[inline]
    pub fn register_sampler_builder(&self, id: i32, builder: &SamplerBuilder) {
        unsafe {
            ffi::sampler_pool_register_sampler_builder(&self.0, id, &builder.0);
        }
    }

    #[inline]
    pub fn get_memory_pool(&self) -> Option<&MemoryPool> {
        unsafe { ffi::sampler_pool_get_memory_pool(&self.0).cast::<MemoryPool>().as_ref() }
    }

    #[inline]
    pub fn get_memory_offset(&self) -> isize {
        unsafe { ffi::sampler_pool_get_memory_offset(&self.0) }
    }

    #[inline]
    pub fn get_size(&self) -> i32 {
        unsafe { ffi::sampler_pool_get_size(&self.0) }
    }
}

impl Drop for SamplerPool {
    fn drop(&mut self) {
        unsafe {
            ffi::sampler_pool_finalize(&mut self.0);
        }
    }
}
