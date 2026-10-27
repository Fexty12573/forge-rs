use core::mem::MaybeUninit;

use sys::nvn::types::NVNtextureView;

use crate::nvn::{Format, TextureDepthStencilMode, TextureSwizzle, TextureTarget, funcs as ffi};

#[repr(transparent)]
pub struct TextureView(pub NVNtextureView);

/// NVN accepts a null view wherever a texture view is taken, meaning "the whole texture".
#[inline]
pub(crate) fn view_ptr(view: Option<&TextureView>) -> *const NVNtextureView {
    view.map_or(core::ptr::null(), |v| &v.0)
}

impl TextureView {
    pub fn new() -> Self {
        let mut v = TextureView(NVNtextureView::default());
        unsafe {
            ffi::texture_view_set_defaults(&mut v.0);
        }

        v
    }

    pub fn set_levels(&mut self, base_level: i32, num_levels: i32) -> &mut Self {
        unsafe {
            ffi::texture_view_set_levels(&mut self.0, base_level, num_levels);
        }
        self
    }

    pub fn set_layers(&mut self, min_layer: i32, num_layers: i32) -> &mut Self {
        unsafe {
            ffi::texture_view_set_layers(&mut self.0, min_layer, num_layers);
        }
        self
    }

    pub fn set_format(&mut self, format: Format) -> &mut Self {
        unsafe {
            ffi::texture_view_set_format(&mut self.0, format);
        }
        self
    }

    pub fn set_swizzle(&mut self, r: TextureSwizzle, g: TextureSwizzle, b: TextureSwizzle, a: TextureSwizzle) -> &mut Self {
        unsafe {
            ffi::texture_view_set_swizzle(&mut self.0, r, g, b, a);
        }
        self
    }

    pub fn set_depth_stencil_mode(&mut self, mode: TextureDepthStencilMode) -> &mut Self {
        unsafe {
            ffi::texture_view_set_depth_stencil_mode(&mut self.0, mode);
        }
        self
    }

    pub fn set_target(&mut self, target: TextureTarget) -> &mut Self {
        unsafe {
            ffi::texture_view_set_target(&mut self.0, target);
        }
        self
    }

    // The getters return `None` when the property was not set on the view.

    pub fn get_levels(&self) -> Option<(i32, i32)> {
        let (mut base, mut num) = (0, 0);
        unsafe { ffi::texture_view_get_levels(&self.0, &mut base, &mut num) }.then_some((base, num))
    }

    pub fn get_layers(&self) -> Option<(i32, i32)> {
        let (mut min, mut num) = (0, 0);
        unsafe { ffi::texture_view_get_layers(&self.0, &mut min, &mut num) }.then_some((min, num))
    }

    pub fn get_format(&self) -> Option<Format> {
        let mut format = MaybeUninit::uninit();
        unsafe { ffi::texture_view_get_format(&self.0, format.as_mut_ptr()).then(|| format.assume_init()) }
    }

    pub fn get_swizzle(&self) -> Option<(TextureSwizzle, TextureSwizzle, TextureSwizzle, TextureSwizzle)> {
        let mut r = MaybeUninit::uninit();
        let mut g = MaybeUninit::uninit();
        let mut b = MaybeUninit::uninit();
        let mut a = MaybeUninit::uninit();
        unsafe {
            ffi::texture_view_get_swizzle(&self.0, r.as_mut_ptr(), g.as_mut_ptr(), b.as_mut_ptr(), a.as_mut_ptr())
                .then(|| (r.assume_init(), g.assume_init(), b.assume_init(), a.assume_init()))
        }
    }

    pub fn get_depth_stencil_mode(&self) -> Option<TextureDepthStencilMode> {
        let mut mode = MaybeUninit::uninit();
        unsafe { ffi::texture_view_get_depth_stencil_mode(&self.0, mode.as_mut_ptr()).then(|| mode.assume_init()) }
    }

    pub fn get_target(&self) -> Option<TextureTarget> {
        let mut target = MaybeUninit::uninit();
        unsafe { ffi::texture_view_get_target(&self.0, target.as_mut_ptr()).then(|| target.assume_init()) }
    }

    pub fn compare(&self, other: &TextureView) -> bool {
        unsafe { ffi::texture_view_compare(&self.0, &other.0) }
    }
}
