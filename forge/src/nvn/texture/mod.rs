use core::cell::UnsafeCell;
use core::ffi::{CStr, c_void};
use core::mem::MaybeUninit;

use sys::nvn::types::{NVNimageHandle, NVNtexture, NVNtextureHandle};

mod builder;
mod view;

pub use builder::TextureBuilder;
pub use view::TextureView;
pub(crate) use view::view_ptr;

use crate::nvn::owned::impl_object;
use crate::nvn::{
    CopyRegion, Format, StorageClass, TextureAddress, TextureDepthStencilMode, TextureFlags_, TextureSwizzle, TextureTarget,
    funcs as ffi, memory_pool::MemoryPool,
};

pub type TextureHandle = NVNtextureHandle;
pub type ImageHandle = NVNimageHandle;

#[repr(transparent)]
pub struct Texture(UnsafeCell<NVNtexture>);

impl_object!(Texture, NVNtexture, ffi::texture_finalize);

impl Texture {
    #[inline]
    pub fn as_ptr(&self) -> *mut NVNtexture {
        self.0.get()
    }

    #[inline]
    pub fn set_debug_label(&mut self, label: &CStr) {
        unsafe {
            ffi::texture_set_debug_label(self.as_ptr(), label.as_ptr());
        }
    }

    #[inline]
    pub fn get_zcull_storage_size(&self) -> usize {
        unsafe { ffi::texture_get_zcull_storage_size(self.as_ptr()) }
    }

    #[inline]
    pub fn get_storage_class(&self) -> StorageClass {
        unsafe { ffi::texture_get_storage_class(self.as_ptr()) }
    }

    #[inline]
    pub fn get_view_offset(&self, view: Option<&TextureView>) -> isize {
        unsafe { ffi::texture_get_view_offset(self.as_ptr(), view_ptr(view)) }
    }

    #[inline]
    pub fn get_flags(&self) -> TextureFlags_ {
        unsafe { ffi::texture_get_flags(self.as_ptr()) }
    }

    #[inline]
    pub fn get_target(&self) -> TextureTarget {
        unsafe { ffi::texture_get_target(self.as_ptr()) }
    }

    #[inline]
    pub fn get_width(&self) -> i32 {
        unsafe { ffi::texture_get_width(self.as_ptr()) }
    }

    #[inline]
    pub fn get_height(&self) -> i32 {
        unsafe { ffi::texture_get_height(self.as_ptr()) }
    }

    #[inline]
    pub fn get_depth(&self) -> i32 {
        unsafe { ffi::texture_get_depth(self.as_ptr()) }
    }

    #[inline]
    pub fn get_levels(&self) -> i32 {
        unsafe { ffi::texture_get_levels(self.as_ptr()) }
    }

    #[inline]
    pub fn get_format(&self) -> Format {
        unsafe { ffi::texture_get_format(self.as_ptr()) }
    }

    #[inline]
    pub fn get_samples(&self) -> i32 {
        unsafe { ffi::texture_get_samples(self.as_ptr()) }
    }

    #[inline]
    pub fn get_swizzle(&self) -> (TextureSwizzle, TextureSwizzle, TextureSwizzle, TextureSwizzle) {
        let mut r = MaybeUninit::uninit();
        let mut g = MaybeUninit::uninit();
        let mut b = MaybeUninit::uninit();
        let mut a = MaybeUninit::uninit();
        unsafe {
            ffi::texture_get_swizzle(self.as_ptr(), r.as_mut_ptr(), g.as_mut_ptr(), b.as_mut_ptr(), a.as_mut_ptr());
            (r.assume_init(), g.assume_init(), b.assume_init(), a.assume_init())
        }
    }

    #[inline]
    pub fn get_depth_stencil_mode(&self) -> TextureDepthStencilMode {
        unsafe { ffi::texture_get_depth_stencil_mode(self.as_ptr()) }
    }

    #[inline]
    pub fn get_stride(&self) -> isize {
        unsafe { ffi::texture_get_stride(self.as_ptr()) }
    }

    #[inline]
    pub fn get_texture_address(&self) -> TextureAddress {
        unsafe { ffi::texture_get_texture_address(self.as_ptr()) }
    }

    /// # Safety
    /// `data` must be valid for reads of the texel data described by `region` and the texture's format.
    #[inline]
    pub unsafe fn write_texels(&self, view: Option<&TextureView>, region: &CopyRegion, data: *const c_void) {
        unsafe {
            ffi::texture_write_texels(self.as_ptr(), view_ptr(view), region, data);
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
            ffi::texture_write_texels_strided(self.as_ptr(), view_ptr(view), region, data, row_stride, image_stride);
        }
    }

    /// # Safety
    /// `data` must be valid for writes of the texel data described by `region` and the texture's format.
    #[inline]
    pub unsafe fn read_texels(&self, view: Option<&TextureView>, region: &CopyRegion, data: *mut c_void) {
        unsafe {
            ffi::texture_read_texels(self.as_ptr(), view_ptr(view), region, data);
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
            ffi::texture_read_texels_strided(self.as_ptr(), view_ptr(view), region, data, row_stride, image_stride);
        }
    }

    #[inline]
    pub fn flush_texels(&self, view: Option<&TextureView>, region: &CopyRegion) {
        unsafe {
            ffi::texture_flush_texels(self.as_ptr(), view_ptr(view), region);
        }
    }

    #[inline]
    pub fn invalidate_texels(&self, view: Option<&TextureView>, region: &CopyRegion) {
        unsafe {
            ffi::texture_invalidate_texels(self.as_ptr(), view_ptr(view), region);
        }
    }

    #[inline]
    pub fn get_memory_pool(&self) -> Option<&MemoryPool> {
        unsafe { ffi::texture_get_memory_pool(self.as_ptr()).cast::<MemoryPool>().as_ref() }
    }

    #[inline]
    pub fn get_memory_offset(&self) -> isize {
        unsafe { ffi::texture_get_memory_offset(self.as_ptr()) }
    }

    #[inline]
    pub fn get_storage_size(&self) -> i32 {
        unsafe { ffi::texture_get_storage_size(self.as_ptr()) }
    }

    #[inline]
    pub fn compare(&self, other: &Texture) -> bool {
        unsafe { ffi::texture_compare(self.as_ptr(), other.as_ptr()) }
    }

    #[inline]
    pub fn get_debug_id(&self) -> u64 {
        unsafe { ffi::texture_get_debug_id(self.as_ptr()) }
    }
}
