use core::mem::MaybeUninit;

use sys::nvn::types::NVNsamplerBuilder;

use crate::nvn::owned::Owned;
use crate::nvn::{
    CompareFunc, CompareMode, MagFilter, MinFilter, SamplerReduction, WrapMode, device::Device, funcs as ffi, sampler::Sampler,
};

#[repr(transparent)]
pub struct SamplerBuilder(pub NVNsamplerBuilder);

impl SamplerBuilder {
    pub fn new() -> Self {
        let mut b = SamplerBuilder(NVNsamplerBuilder::default());
        unsafe {
            ffi::sampler_builder_set_defaults(&mut b.0);
        }

        b
    }

    pub fn set_device(&mut self, device: &Device) -> &mut Self {
        unsafe {
            ffi::sampler_builder_set_device(&mut self.0, device.as_ptr());
        }
        self
    }

    pub fn set_min_mag_filter(&mut self, min: MinFilter, mag: MagFilter) -> &mut Self {
        unsafe {
            ffi::sampler_builder_set_min_mag_filter(&mut self.0, min, mag);
        }
        self
    }

    pub fn set_wrap_mode(&mut self, s: WrapMode, t: WrapMode, r: WrapMode) -> &mut Self {
        unsafe {
            ffi::sampler_builder_set_wrap_mode(&mut self.0, s, t, r);
        }
        self
    }

    pub fn set_lod_clamp(&mut self, min: f32, max: f32) -> &mut Self {
        unsafe {
            ffi::sampler_builder_set_lod_clamp(&mut self.0, min, max);
        }
        self
    }

    pub fn set_lod_bias(&mut self, bias: f32) -> &mut Self {
        unsafe {
            ffi::sampler_builder_set_lod_bias(&mut self.0, bias);
        }
        self
    }

    pub fn set_compare(&mut self, mode: CompareMode, func: CompareFunc) -> &mut Self {
        unsafe {
            ffi::sampler_builder_set_compare(&mut self.0, mode, func);
        }
        self
    }

    pub fn set_border_color(&mut self, color: &[f32; 4]) -> &mut Self {
        unsafe {
            ffi::sampler_builder_set_border_color(&mut self.0, color.as_ptr());
        }
        self
    }

    pub fn set_border_color_i(&mut self, color: &[i32; 4]) -> &mut Self {
        unsafe {
            ffi::sampler_builder_set_border_colori(&mut self.0, color.as_ptr());
        }
        self
    }

    pub fn set_border_color_ui(&mut self, color: &[u32; 4]) -> &mut Self {
        unsafe {
            ffi::sampler_builder_set_border_colorui(&mut self.0, color.as_ptr());
        }
        self
    }

    pub fn set_max_anisotropy(&mut self, max_aniso: f32) -> &mut Self {
        unsafe {
            ffi::sampler_builder_set_max_anisotropy(&mut self.0, max_aniso);
        }
        self
    }

    pub fn set_reduction_filter(&mut self, filter: SamplerReduction) -> &mut Self {
        unsafe {
            ffi::sampler_builder_set_reduction_filter(&mut self.0, filter);
        }
        self
    }

    pub fn set_lod_snap(&mut self, lod_snap: f32) -> &mut Self {
        unsafe {
            ffi::sampler_builder_set_lod_snap(&mut self.0, lod_snap);
        }
        self
    }

    pub fn get_min_mag_filter(&self) -> (MinFilter, MagFilter) {
        let mut min = MaybeUninit::uninit();
        let mut mag = MaybeUninit::uninit();
        unsafe {
            ffi::sampler_builder_get_min_mag_filter(&self.0, min.as_mut_ptr(), mag.as_mut_ptr());
            (min.assume_init(), mag.assume_init())
        }
    }

    pub fn get_wrap_mode(&self) -> (WrapMode, WrapMode, WrapMode) {
        let mut s = MaybeUninit::uninit();
        let mut t = MaybeUninit::uninit();
        let mut r = MaybeUninit::uninit();
        unsafe {
            ffi::sampler_builder_get_wrap_mode(&self.0, s.as_mut_ptr(), t.as_mut_ptr(), r.as_mut_ptr());
            (s.assume_init(), t.assume_init(), r.assume_init())
        }
    }

    pub fn get_lod_clamp(&self) -> (f32, f32) {
        let (mut min, mut max) = (0.0, 0.0);
        unsafe {
            ffi::sampler_builder_get_lod_clamp(&self.0, &mut min, &mut max);
        }
        (min, max)
    }

    pub fn get_lod_bias(&self) -> f32 {
        unsafe { ffi::sampler_builder_get_lod_bias(&self.0) }
    }

    pub fn get_compare(&self) -> (CompareMode, CompareFunc) {
        let mut mode = MaybeUninit::uninit();
        let mut func = MaybeUninit::uninit();
        unsafe {
            ffi::sampler_builder_get_compare(&self.0, mode.as_mut_ptr(), func.as_mut_ptr());
            (mode.assume_init(), func.assume_init())
        }
    }

    pub fn get_border_color(&self) -> [f32; 4] {
        let mut color = [0.0; 4];
        unsafe {
            ffi::sampler_builder_get_border_color(&self.0, color.as_mut_ptr());
        }
        color
    }

    pub fn get_border_color_i(&self) -> [i32; 4] {
        let mut color = [0; 4];
        unsafe {
            ffi::sampler_builder_get_border_colori(&self.0, color.as_mut_ptr());
        }
        color
    }

    pub fn get_border_color_ui(&self) -> [u32; 4] {
        let mut color = [0; 4];
        unsafe {
            ffi::sampler_builder_get_border_colorui(&self.0, color.as_mut_ptr());
        }
        color
    }

    pub fn get_max_anisotropy(&self) -> f32 {
        unsafe { ffi::sampler_builder_get_max_anisotropy(&self.0) }
    }

    pub fn get_reduction_filter(&self) -> SamplerReduction {
        unsafe { ffi::sampler_builder_get_reduction_filter(&self.0) }
    }

    pub fn get_lod_snap(&self) -> f32 {
        unsafe { ffi::sampler_builder_get_lod_snap(&self.0) }
    }

    pub fn build(&self) -> Option<Owned<Sampler>> {
        Owned::<Sampler>::try_new(|sampler| unsafe { ffi::sampler_initialize(sampler.as_ptr(), &self.0) })
    }
}
