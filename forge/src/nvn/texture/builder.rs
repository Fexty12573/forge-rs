use core::ffi::c_void;
use core::mem::MaybeUninit;

use sys::nvn::types::NVNtextureBuilder;

use crate::nvn::owned::Owned;
use crate::nvn::{
    Format, PackagedTextureLayout, StorageClass, TextureDepthStencilMode, TextureFlags_, TextureSwizzle, TextureTarget,
    device::Device, funcs as ffi, memory_pool::MemoryPool, texture::Texture,
};

#[repr(transparent)]
pub struct TextureBuilder(pub NVNtextureBuilder);

impl TextureBuilder {
    pub fn new() -> Self {
        let mut b = TextureBuilder(NVNtextureBuilder::default());
        unsafe {
            ffi::texture_builder_set_defaults(&mut b.0);
        }

        b
    }

    pub fn set_device(&mut self, device: &Device) -> &mut Self {
        unsafe {
            ffi::texture_builder_set_device(&mut self.0, device.as_ptr());
        }
        self
    }

    pub fn set_flags(&mut self, flags: TextureFlags_) -> &mut Self {
        unsafe {
            ffi::texture_builder_set_flags(&mut self.0, flags);
        }
        self
    }

    pub fn set_target(&mut self, target: TextureTarget) -> &mut Self {
        unsafe {
            ffi::texture_builder_set_target(&mut self.0, target);
        }
        self
    }

    pub fn set_width(&mut self, width: i32) -> &mut Self {
        unsafe {
            ffi::texture_builder_set_width(&mut self.0, width);
        }
        self
    }

    pub fn set_height(&mut self, height: i32) -> &mut Self {
        unsafe {
            ffi::texture_builder_set_height(&mut self.0, height);
        }
        self
    }

    pub fn set_depth(&mut self, depth: i32) -> &mut Self {
        unsafe {
            ffi::texture_builder_set_depth(&mut self.0, depth);
        }
        self
    }

    pub fn set_size_1d(&mut self, size: i32) -> &mut Self {
        unsafe {
            ffi::texture_builder_set_size_1d(&mut self.0, size);
        }
        self
    }

    pub fn set_size_2d(&mut self, width: i32, height: i32) -> &mut Self {
        unsafe {
            ffi::texture_builder_set_size_2d(&mut self.0, width, height);
        }
        self
    }

    pub fn set_size_3d(&mut self, width: i32, height: i32, depth: i32) -> &mut Self {
        unsafe {
            ffi::texture_builder_set_size_3d(&mut self.0, width, height, depth);
        }
        self
    }

    pub fn set_levels(&mut self, num_levels: i32) -> &mut Self {
        unsafe {
            ffi::texture_builder_set_levels(&mut self.0, num_levels);
        }
        self
    }

    pub fn set_format(&mut self, format: Format) -> &mut Self {
        unsafe {
            ffi::texture_builder_set_format(&mut self.0, format);
        }
        self
    }

    pub fn set_samples(&mut self, samples: i32) -> &mut Self {
        unsafe {
            ffi::texture_builder_set_samples(&mut self.0, samples);
        }
        self
    }

    pub fn set_swizzle(&mut self, r: TextureSwizzle, g: TextureSwizzle, b: TextureSwizzle, a: TextureSwizzle) -> &mut Self {
        unsafe {
            ffi::texture_builder_set_swizzle(&mut self.0, r, g, b, a);
        }
        self
    }

    pub fn set_depth_stencil_mode(&mut self, mode: TextureDepthStencilMode) -> &mut Self {
        unsafe {
            ffi::texture_builder_set_depth_stencil_mode(&mut self.0, mode);
        }
        self
    }

    pub fn set_storage(&mut self, pool: &MemoryPool, offset: isize) -> &mut Self {
        unsafe {
            ffi::texture_builder_set_storage(&mut self.0, pool.as_ptr(), offset);
        }
        self
    }

    /// # Safety
    /// `data` must point to packaged texture data that stays valid until the texture is initialized.
    pub unsafe fn set_packaged_texture_data(&mut self, data: *const c_void) -> &mut Self {
        unsafe {
            ffi::texture_builder_set_packaged_texture_data(&mut self.0, data);
        }
        self
    }

    pub fn set_packaged_texture_layout(&mut self, layout: &PackagedTextureLayout) -> &mut Self {
        unsafe {
            ffi::texture_builder_set_packaged_texture_layout(&mut self.0, layout);
        }
        self
    }

    pub fn set_stride(&mut self, stride: isize) -> &mut Self {
        unsafe {
            ffi::texture_builder_set_stride(&mut self.0, stride);
        }
        self
    }

    pub fn set_gl_texture_name(&mut self, name: u32) -> &mut Self {
        unsafe {
            ffi::texture_builder_set_gl_texture_name(&mut self.0, name);
        }
        self
    }

    pub fn get_storage_size(&self) -> usize {
        unsafe { ffi::texture_builder_get_storage_size(&self.0) }
    }

    pub fn get_storage_alignment(&self) -> usize {
        unsafe { ffi::texture_builder_get_storage_alignment(&self.0) }
    }

    pub fn get_storage_class(&self) -> StorageClass {
        unsafe { ffi::texture_builder_get_storage_class(&self.0) }
    }

    pub fn get_flags(&self) -> TextureFlags_ {
        unsafe { ffi::texture_builder_get_flags(&self.0) }
    }

    pub fn get_target(&self) -> TextureTarget {
        unsafe { ffi::texture_builder_get_target(&self.0) }
    }

    pub fn get_width(&self) -> i32 {
        unsafe { ffi::texture_builder_get_width(&self.0) }
    }

    pub fn get_height(&self) -> i32 {
        unsafe { ffi::texture_builder_get_height(&self.0) }
    }

    pub fn get_depth(&self) -> i32 {
        unsafe { ffi::texture_builder_get_depth(&self.0) }
    }

    pub fn get_levels(&self) -> i32 {
        unsafe { ffi::texture_builder_get_levels(&self.0) }
    }

    pub fn get_format(&self) -> Format {
        unsafe { ffi::texture_builder_get_format(&self.0) }
    }

    pub fn get_samples(&self) -> i32 {
        unsafe { ffi::texture_builder_get_samples(&self.0) }
    }

    pub fn get_swizzle(&self) -> (TextureSwizzle, TextureSwizzle, TextureSwizzle, TextureSwizzle) {
        let mut r = MaybeUninit::uninit();
        let mut g = MaybeUninit::uninit();
        let mut b = MaybeUninit::uninit();
        let mut a = MaybeUninit::uninit();
        unsafe {
            ffi::texture_builder_get_swizzle(&self.0, r.as_mut_ptr(), g.as_mut_ptr(), b.as_mut_ptr(), a.as_mut_ptr());
            (r.assume_init(), g.assume_init(), b.assume_init(), a.assume_init())
        }
    }

    pub fn get_depth_stencil_mode(&self) -> TextureDepthStencilMode {
        unsafe { ffi::texture_builder_get_depth_stencil_mode(&self.0) }
    }

    pub fn get_packaged_texture_data(&self) -> *const c_void {
        unsafe { ffi::texture_builder_get_packaged_texture_data(&self.0) }
    }

    pub fn get_stride(&self) -> isize {
        unsafe { ffi::texture_builder_get_stride(&self.0) }
    }

    pub fn get_gl_texture_name(&self) -> u32 {
        unsafe { ffi::texture_builder_get_gl_texture_name(&self.0) }
    }

    pub fn get_zcull_storage_size(&self) -> usize {
        unsafe { ffi::texture_builder_get_zcull_storage_size(&self.0) }
    }

    pub fn get_memory_pool(&self) -> Option<&MemoryPool> {
        unsafe { ffi::texture_builder_get_memory_pool(&self.0).cast::<MemoryPool>().as_ref() }
    }

    pub fn get_memory_offset(&self) -> isize {
        unsafe { ffi::texture_builder_get_memory_offset(&self.0) }
    }

    pub fn build(&self) -> Option<Owned<Texture>> {
        Owned::<Texture>::try_new(|texture| unsafe { ffi::texture_initialize(texture.as_ptr(), &self.0) })
    }
}
