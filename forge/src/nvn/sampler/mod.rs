use core::ffi::CStr;
use core::mem::MaybeUninit;

use sys::nvn::types::NVNsampler;

mod builder;

pub use builder::SamplerBuilder;

use crate::nvn::{CompareFunc, CompareMode, MagFilter, MinFilter, SamplerReduction, WrapMode, funcs as ffi};

#[repr(transparent)]
pub struct Sampler(pub NVNsampler);

impl Sampler {
    #[inline]
    pub fn set_debug_label(&mut self, label: &CStr) {
        unsafe {
            ffi::sampler_set_debug_label(&mut self.0, label.as_ptr());
        }
    }

    #[inline]
    pub fn get_min_mag_filter(&self) -> (MinFilter, MagFilter) {
        let mut min = MaybeUninit::uninit();
        let mut mag = MaybeUninit::uninit();
        unsafe {
            ffi::sampler_get_min_mag_filter(&self.0, min.as_mut_ptr(), mag.as_mut_ptr());
            (min.assume_init(), mag.assume_init())
        }
    }

    #[inline]
    pub fn get_wrap_mode(&self) -> (WrapMode, WrapMode, WrapMode) {
        let mut s = MaybeUninit::uninit();
        let mut t = MaybeUninit::uninit();
        let mut r = MaybeUninit::uninit();
        unsafe {
            ffi::sampler_get_wrap_mode(&self.0, s.as_mut_ptr(), t.as_mut_ptr(), r.as_mut_ptr());
            (s.assume_init(), t.assume_init(), r.assume_init())
        }
    }

    #[inline]
    pub fn get_lod_clamp(&self) -> (f32, f32) {
        let (mut min, mut max) = (0.0, 0.0);
        unsafe {
            ffi::sampler_get_lod_clamp(&self.0, &mut min, &mut max);
        }
        (min, max)
    }

    #[inline]
    pub fn get_lod_bias(&self) -> f32 {
        unsafe { ffi::sampler_get_lod_bias(&self.0) }
    }

    #[inline]
    pub fn get_compare(&self) -> (CompareMode, CompareFunc) {
        let mut mode = MaybeUninit::uninit();
        let mut func = MaybeUninit::uninit();
        unsafe {
            ffi::sampler_get_compare(&self.0, mode.as_mut_ptr(), func.as_mut_ptr());
            (mode.assume_init(), func.assume_init())
        }
    }

    #[inline]
    pub fn get_border_color(&self) -> [f32; 4] {
        let mut color = [0.0; 4];
        unsafe {
            ffi::sampler_get_border_color(&self.0, color.as_mut_ptr());
        }
        color
    }

    #[inline]
    pub fn get_border_color_i(&self) -> [i32; 4] {
        let mut color = [0; 4];
        unsafe {
            ffi::sampler_get_border_colori(&self.0, color.as_mut_ptr());
        }
        color
    }

    #[inline]
    pub fn get_border_color_ui(&self) -> [u32; 4] {
        let mut color = [0; 4];
        unsafe {
            ffi::sampler_get_border_colorui(&self.0, color.as_mut_ptr());
        }
        color
    }

    #[inline]
    pub fn get_max_anisotropy(&self) -> f32 {
        unsafe { ffi::sampler_get_max_anisotropy(&self.0) }
    }

    #[inline]
    pub fn get_reduction_filter(&self) -> SamplerReduction {
        unsafe { ffi::sampler_get_reduction_filter(&self.0) }
    }

    #[inline]
    pub fn compare(&self, other: &Sampler) -> bool {
        unsafe { ffi::sampler_compare(&self.0, &other.0) }
    }

    #[inline]
    pub fn get_debug_id(&self) -> u64 {
        unsafe { ffi::sampler_get_debug_id(&self.0) }
    }
}

impl Drop for Sampler {
    fn drop(&mut self) {
        unsafe {
            ffi::sampler_finalize(&mut self.0);
        }
    }
}
