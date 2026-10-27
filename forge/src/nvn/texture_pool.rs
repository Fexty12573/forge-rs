use core::cell::UnsafeCell;
use core::ffi::CStr;

use sys::nvn::types::NVNtexturePool;

use crate::nvn::owned::{Owned, impl_object};
use crate::nvn::{
    funcs as ffi,
    memory_pool::MemoryPool,
    texture::{Texture, TextureView, view_ptr},
};

#[repr(transparent)]
pub struct TexturePool(UnsafeCell<NVNtexturePool>);

impl_object!(TexturePool, NVNtexturePool, ffi::texture_pool_finalize);

impl TexturePool {
    #[inline]
    pub fn as_ptr(&self) -> *mut NVNtexturePool {
        self.0.get()
    }

    pub fn new(memory_pool: &MemoryPool, offset: isize, num_descriptors: i32) -> Option<Owned<Self>> {
        Owned::<Self>::try_new(|pool| unsafe {
            ffi::texture_pool_initialize(pool.as_ptr(), memory_pool.as_ptr(), offset, num_descriptors)
        })
    }

    #[inline]
    pub fn set_debug_label(&mut self, label: &CStr) {
        unsafe {
            ffi::texture_pool_set_debug_label(self.as_ptr(), label.as_ptr());
        }
    }

    #[inline]
    pub fn register_texture(&self, id: i32, texture: &Texture, view: Option<&TextureView>) {
        unsafe {
            ffi::texture_pool_register_texture(self.as_ptr(), id, texture.as_ptr(), view_ptr(view));
        }
    }

    #[inline]
    pub fn register_image(&self, id: i32, texture: &Texture, view: Option<&TextureView>) {
        unsafe {
            ffi::texture_pool_register_image(self.as_ptr(), id, texture.as_ptr(), view_ptr(view));
        }
    }

    #[inline]
    pub fn get_memory_pool(&self) -> Option<&MemoryPool> {
        unsafe { ffi::texture_pool_get_memory_pool(self.as_ptr()).cast::<MemoryPool>().as_ref() }
    }

    #[inline]
    pub fn get_memory_offset(&self) -> isize {
        unsafe { ffi::texture_pool_get_memory_offset(self.as_ptr()) }
    }

    #[inline]
    pub fn get_size(&self) -> i32 {
        unsafe { ffi::texture_pool_get_size(self.as_ptr()) }
    }
}
