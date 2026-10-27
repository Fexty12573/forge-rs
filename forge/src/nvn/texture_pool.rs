use core::ffi::CStr;

use sys::nvn::types::NVNtexturePool;

use crate::nvn::{
    funcs as ffi,
    memory_pool::MemoryPool,
    texture::{Texture, TextureView, view_ptr},
};

#[repr(transparent)]
pub struct TexturePool(pub NVNtexturePool);

impl TexturePool {
    pub fn new(memory_pool: &MemoryPool, offset: isize, num_descriptors: i32) -> Self {
        let mut pool = NVNtexturePool::default();
        unsafe {
            ffi::texture_pool_initialize(&mut pool, &memory_pool.0, offset, num_descriptors);
        }

        Self(pool)
    }

    #[inline]
    pub fn set_debug_label(&mut self, label: &CStr) {
        unsafe {
            ffi::texture_pool_set_debug_label(&mut self.0, label.as_ptr());
        }
    }

    #[inline]
    pub fn register_texture(&self, id: i32, texture: &Texture, view: Option<&TextureView>) {
        unsafe {
            ffi::texture_pool_register_texture(&self.0, id, &texture.0, view_ptr(view));
        }
    }

    #[inline]
    pub fn register_image(&self, id: i32, texture: &Texture, view: Option<&TextureView>) {
        unsafe {
            ffi::texture_pool_register_image(&self.0, id, &texture.0, view_ptr(view));
        }
    }

    #[inline]
    pub fn get_memory_pool(&self) -> Option<&MemoryPool> {
        unsafe { ffi::texture_pool_get_memory_pool(&self.0).cast::<MemoryPool>().as_ref() }
    }

    #[inline]
    pub fn get_memory_offset(&self) -> isize {
        unsafe { ffi::texture_pool_get_memory_offset(&self.0) }
    }

    #[inline]
    pub fn get_size(&self) -> i32 {
        unsafe { ffi::texture_pool_get_size(&self.0) }
    }
}

impl Drop for TexturePool {
    fn drop(&mut self) {
        unsafe {
            ffi::texture_pool_finalize(&mut self.0);
        }
    }
}
