use core::ffi::{CStr, c_void};
use core::mem::MaybeUninit;

use sys::nvn::types::{NVNimageHandle, NVNtexture, NVNtextureHandle};

mod builder;
mod view;

pub use builder::TextureBuilder;
pub use view::TextureView;
pub(crate) use view::view_ptr;

use crate::nvn::{
    CopyRegion, Format, StorageClass, TextureAddress, TextureDepthStencilMode, TextureFlags_, TextureSwizzle, TextureTarget,
    funcs as ffi, memory_pool::MemoryPool,
};

pub type TextureHandle = NVNtextureHandle;
pub type ImageHandle = NVNimageHandle;

#[repr(transparent)]
pub struct Texture(pub NVNtexture);

impl Texture {
    #[inline]
    pub fn set_debug_label(&mut self, label: &CStr) {
        unsafe {
            ffi::texture_set_debug_label(&mut self.0, label.as_ptr());
        }
    }

    #[inline]
    pub fn get_zcull_storage_size(&self) -> usize {
        unsafe { ffi::texture_get_zcull_storage_size(&self.0) }
    }

    #[inline]
    pub fn get_storage_class(&self) -> StorageClass {
        unsafe { ffi::texture_get_storage_class(&self.0) }
    }

    #[inline]
    pub fn get_view_offset(&self, view: Option<&TextureView>) -> isize {
        unsafe { ffi::texture_get_view_offset(&self.0, view_ptr(view)) }
    }

    #[inline]
    pub fn get_flags(&self) -> TextureFlags_ {
        unsafe { ffi::texture_get_flags(&self.0) }
    }

    #[inline]
    pub fn get_target(&self) -> TextureTarget {
        unsafe { ffi::texture_get_target(&self.0) }
    }

    #[inline]
    pub fn get_width(&self) -> i32 {
        unsafe { ffi::texture_get_width(&self.0) }
    }

    #[inline]
    pub fn get_height(&self) -> i32 {
        unsafe { ffi::texture_get_height(&self.0) }
    }

    #[inline]
    pub fn get_depth(&self) -> i32 {
        unsafe { ffi::texture_get_depth(&self.0) }
    }

    #[inline]
    pub fn get_levels(&self) -> i32 {
        unsafe { ffi::texture_get_levels(&self.0) }
    }

    #[inline]
    pub fn get_format(&self) -> Format {
        unsafe { ffi::texture_get_format(&self.0) }
    }

    #[inline]
    pub fn get_samples(&self) -> i32 {
        unsafe { ffi::texture_get_samples(&self.0) }
    }

    #[inline]
    pub fn get_swizzle(&self) -> (TextureSwizzle, TextureSwizzle, TextureSwizzle, TextureSwizzle) {
        let mut r = MaybeUninit::uninit();
        let mut g = MaybeUninit::uninit();
        let mut b = MaybeUninit::uninit();
        let mut a = MaybeUninit::uninit();
        unsafe {
            ffi::texture_get_swizzle(&self.0, r.as_mut_ptr(), g.as_mut_ptr(), b.as_mut_ptr(), a.as_mut_ptr());
            (r.assume_init(), g.assume_init(), b.assume_init(), a.assume_init())
        }
    }

    #[inline]
    pub fn get_depth_stencil_mode(&self) -> TextureDepthStencilMode {
        unsafe { ffi::texture_get_depth_stencil_mode(&self.0) }
    }

    #[inline]
    pub fn get_stride(&self) -> isize {
        unsafe { ffi::texture_get_stride(&self.0) }
    }

    #[inline]
    pub fn get_texture_address(&self) -> TextureAddress {
        unsafe { ffi::texture_get_texture_address(&self.0) }
    }

    /// # Safety
    /// `data` must be valid for reads of the texel data described by `region` and the texture's format.
    #[inline]
    pub unsafe fn write_texels(&self, view: Option<&TextureView>, region: &CopyRegion, data: *const c_void) {
        unsafe {
            ffi::texture_write_texels(&self.0, view_ptr(view), region, data);
        }
    }

    /// # Safety
    /// `data` must be valid for reads of the texel data described by `region` and the texture's format.
    #[inline]
    pub unsafe fn write_texels_strided(
        &self,
        view: Option<&TextureView>,
        region: &CopyRegion,
        data: *const c_void,
        row_stride: isize,
        image_stride: isize,
    ) {
        unsafe {
            ffi::texture_write_texels_strided(&self.0, view_ptr(view), region, data, row_stride, image_stride);
        }
    }

    /// # Safety
    /// `data` must be valid for writes of the texel data described by `region` and the texture's format.
    #[inline]
    pub unsafe fn read_texels(&self, view: Option<&TextureView>, region: &CopyRegion, data: *mut c_void) {
        unsafe {
            ffi::texture_read_texels(&self.0, view_ptr(view), region, data);
        }
    }

    /// # Safety
    /// `data` must be valid for writes of the texel data described by `region` and the texture's format.
    #[inline]
    pub unsafe fn read_texels_strided(
        &self,
        view: Option<&TextureView>,
        region: &CopyRegion,
        data: *mut c_void,
        row_stride: isize,
        image_stride: isize,
    ) {
        unsafe {
            ffi::texture_read_texels_strided(&self.0, view_ptr(view), region, data, row_stride, image_stride);
        }
    }

    #[inline]
    pub fn flush_texels(&self, view: Option<&TextureView>, region: &CopyRegion) {
        unsafe {
            ffi::texture_flush_texels(&self.0, view_ptr(view), region);
        }
    }

    #[inline]
    pub fn invalidate_texels(&self, view: Option<&TextureView>, region: &CopyRegion) {
        unsafe {
            ffi::texture_invalidate_texels(&self.0, view_ptr(view), region);
        }
    }

    #[inline]
    pub fn get_memory_pool(&self) -> Option<&MemoryPool> {
        unsafe { ffi::texture_get_memory_pool(&self.0).cast::<MemoryPool>().as_ref() }
    }

    #[inline]
    pub fn get_memory_offset(&self) -> isize {
        unsafe { ffi::texture_get_memory_offset(&self.0) }
    }

    #[inline]
    pub fn get_storage_size(&self) -> i32 {
        unsafe { ffi::texture_get_storage_size(&self.0) }
    }

    #[inline]
    pub fn compare(&self, other: &Texture) -> bool {
        unsafe { ffi::texture_compare(&self.0, &other.0) }
    }

    #[inline]
    pub fn get_debug_id(&self) -> u64 {
        unsafe { ffi::texture_get_debug_id(&self.0) }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            ffi::texture_finalize(&mut self.0);
        }
    }
}
