use core::cell::UnsafeCell;
use core::ffi::CStr;

use sys::nvn::types::NVNsamplerPool;

use crate::nvn::owned::{Owned, impl_object};
use crate::nvn::{
    funcs as ffi,
    memory_pool::MemoryPool,
    sampler::{Sampler, SamplerBuilder},
};

#[repr(transparent)]
pub struct SamplerPool(UnsafeCell<NVNsamplerPool>);

impl_object!(SamplerPool, NVNsamplerPool, ffi::sampler_pool_finalize);

impl SamplerPool {
    #[inline]
    pub fn as_ptr(&self) -> *mut NVNsamplerPool {
        self.0.get()
    }

    pub fn new(memory_pool: &MemoryPool, offset: isize, num_descriptors: i32) -> Option<Owned<Self>> {
        Owned::<Self>::try_new(|pool| unsafe {
            ffi::sampler_pool_initialize(pool.as_ptr(), memory_pool.as_ptr(), offset, num_descriptors)
        })
    }

    #[inline]
    pub fn set_debug_label(&mut self, label: &CStr) {
        unsafe {
            ffi::sampler_pool_set_debug_label(self.as_ptr(), label.as_ptr());
        }
    }

    #[inline]
    pub fn register_sampler(&self, id: i32, sampler: &Sampler) {
        unsafe {
            ffi::sampler_pool_register_sampler(self.as_ptr(), id, sampler.as_ptr());
        }
    }

    #[inline]
    pub fn register_sampler_builder(&self, id: i32, builder: &SamplerBuilder) {
        unsafe {
            ffi::sampler_pool_register_sampler_builder(self.as_ptr(), id, &builder.0);
        }
    }

    #[inline]
    pub fn get_memory_pool(&self) -> Option<&MemoryPool> {
        unsafe { ffi::sampler_pool_get_memory_pool(self.as_ptr()).cast::<MemoryPool>().as_ref() }
    }

    #[inline]
    pub fn get_memory_offset(&self) -> isize {
        unsafe { ffi::sampler_pool_get_memory_offset(self.as_ptr()) }
    }

    #[inline]
    pub fn get_size(&self) -> i32 {
        unsafe { ffi::sampler_pool_get_size(self.as_ptr()) }
    }
}
