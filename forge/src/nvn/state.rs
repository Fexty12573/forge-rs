use core::mem::MaybeUninit;

use sys::nvn::types::{
    NVNblendState, NVNchannelMaskState, NVNcolorState, NVNdepthStencilState, NVNmultisampleState, NVNpolygonState,
    NVNvertexAttribState, NVNvertexStreamState,
};

use crate::nvn::{
    AlphaFunc, BlendAdvancedMode, BlendAdvancedOverlap, BlendEquation, BlendFunc, CoverageModulationMode, DepthFunc, Face,
    Format, FrontFace, LogicOp, PolygonMode, PolygonOffsetEnable, StencilFunc, StencilOp, funcs as ffi,
};

#[repr(transparent)]
pub struct BlendState(pub NVNblendState);

impl BlendState {
    pub fn new() -> Self {
        let mut s = BlendState(NVNblendState::default());
        unsafe {
            ffi::blend_state_set_defaults(&mut s.0);
        }

        s
    }

    pub fn set_blend_target(&mut self, target: i32) -> &mut Self {
        unsafe {
            ffi::blend_state_set_blend_target(&mut self.0, target);
        }
        self
    }

    pub fn set_blend_func(
        &mut self,
        src_func: BlendFunc,
        dst_func: BlendFunc,
        src_func_alpha: BlendFunc,
        dst_func_alpha: BlendFunc,
    ) -> &mut Self {
        unsafe {
            ffi::blend_state_set_blend_func(&mut self.0, src_func, dst_func, src_func_alpha, dst_func_alpha);
        }
        self
    }

    pub fn set_blend_equation(&mut self, mode_rgb: BlendEquation, mode_alpha: BlendEquation) -> &mut Self {
        unsafe {
            ffi::blend_state_set_blend_equation(&mut self.0, mode_rgb, mode_alpha);
        }
        self
    }

    pub fn set_advanced_mode(&mut self, mode: BlendAdvancedMode) -> &mut Self {
        unsafe {
            ffi::blend_state_set_advanced_mode(&mut self.0, mode);
        }
        self
    }

    pub fn set_advanced_overlap(&mut self, overlap: BlendAdvancedOverlap) -> &mut Self {
        unsafe {
            ffi::blend_state_set_advanced_overlap(&mut self.0, overlap);
        }
        self
    }

    pub fn set_advanced_premultiplied_src(&mut self, premultiplied: bool) -> &mut Self {
        unsafe {
            ffi::blend_state_set_advanced_premultiplied_src(&mut self.0, premultiplied);
        }
        self
    }

    pub fn set_advanced_normalized_dst(&mut self, normalized: bool) -> &mut Self {
        unsafe {
            ffi::blend_state_set_advanced_normalized_dst(&mut self.0, normalized);
        }
        self
    }

    pub fn get_blend_target(&self) -> i32 {
        unsafe { ffi::blend_state_get_blend_target(&self.0) }
    }

    pub fn get_blend_func(&self) -> (BlendFunc, BlendFunc, BlendFunc, BlendFunc) {
        let mut src = MaybeUninit::uninit();
        let mut dst = MaybeUninit::uninit();
        let mut src_alpha = MaybeUninit::uninit();
        let mut dst_alpha = MaybeUninit::uninit();
        unsafe {
            ffi::blend_state_get_blend_func(
                &self.0,
                src.as_mut_ptr(),
                dst.as_mut_ptr(),
                src_alpha.as_mut_ptr(),
                dst_alpha.as_mut_ptr(),
            );
            (
                src.assume_init(),
                dst.assume_init(),
                src_alpha.assume_init(),
                dst_alpha.assume_init(),
            )
        }
    }

    pub fn get_blend_equation(&self) -> (BlendEquation, BlendEquation) {
        let mut rgb = MaybeUninit::uninit();
        let mut alpha = MaybeUninit::uninit();
        unsafe {
            ffi::blend_state_get_blend_equation(&self.0, rgb.as_mut_ptr(), alpha.as_mut_ptr());
            (rgb.assume_init(), alpha.assume_init())
        }
    }

    pub fn get_advanced_mode(&self) -> BlendAdvancedMode {
        unsafe { ffi::blend_state_get_advanced_mode(&self.0) }
    }

    pub fn get_advanced_overlap(&self) -> BlendAdvancedOverlap {
        unsafe { ffi::blend_state_get_advanced_overlap(&self.0) }
    }

    pub fn get_advanced_premultiplied_src(&self) -> bool {
        unsafe { ffi::blend_state_get_advanced_premultiplied_src(&self.0) }
    }

    pub fn get_advanced_normalized_dst(&self) -> bool {
        unsafe { ffi::blend_state_get_advanced_normalized_dst(&self.0) }
    }
}

#[repr(transparent)]
pub struct ColorState(pub NVNcolorState);

impl ColorState {
    pub fn new() -> Self {
        let mut s = ColorState(NVNcolorState::default());
        unsafe {
            ffi::color_state_set_defaults(&mut s.0);
        }

        s
    }

    pub fn set_blend_enable(&mut self, index: i32, enable: bool) -> &mut Self {
        unsafe {
            ffi::color_state_set_blend_enable(&mut self.0, index, enable);
        }
        self
    }

    pub fn set_logic_op(&mut self, logic_op: LogicOp) -> &mut Self {
        unsafe {
            ffi::color_state_set_logic_op(&mut self.0, logic_op);
        }
        self
    }

    pub fn set_alpha_test(&mut self, alpha_func: AlphaFunc) -> &mut Self {
        unsafe {
            ffi::color_state_set_alpha_test(&mut self.0, alpha_func);
        }
        self
    }

    pub fn get_blend_enable(&self, index: i32) -> bool {
        unsafe { ffi::color_state_get_blend_enable(&self.0, index) }
    }

    pub fn get_logic_op(&self) -> LogicOp {
        unsafe { ffi::color_state_get_logic_op(&self.0) }
    }

    pub fn get_alpha_test(&self) -> AlphaFunc {
        unsafe { ffi::color_state_get_alpha_test(&self.0) }
    }
}

#[repr(transparent)]
pub struct ChannelMaskState(pub NVNchannelMaskState);

impl ChannelMaskState {
    pub fn new() -> Self {
        let mut s = ChannelMaskState(NVNchannelMaskState::default());
        unsafe {
            ffi::channel_mask_state_set_defaults(&mut s.0);
        }

        s
    }

    pub fn set_channel_mask(&mut self, index: i32, r: bool, g: bool, b: bool, a: bool) -> &mut Self {
        unsafe {
            ffi::channel_mask_state_set_channel_mask(&mut self.0, index, r, g, b, a);
        }
        self
    }

    pub fn get_channel_mask(&self, index: i32) -> (bool, bool, bool, bool) {
        let (mut r, mut g, mut b, mut a) = (false, false, false, false);
        unsafe {
            ffi::channel_mask_state_get_channel_mask(&self.0, index, &mut r, &mut g, &mut b, &mut a);
        }
        (r, g, b, a)
    }
}

#[repr(transparent)]
pub struct MultisampleState(pub NVNmultisampleState);

impl MultisampleState {
    pub fn new() -> Self {
        let mut s = MultisampleState(NVNmultisampleState::default());
        unsafe {
            ffi::multisample_state_set_defaults(&mut s.0);
        }

        s
    }

    pub fn set_multisample_enable(&mut self, enable: bool) -> &mut Self {
        unsafe {
            ffi::multisample_state_set_multisample_enable(&mut self.0, enable);
        }
        self
    }

    pub fn set_samples(&mut self, samples: i32) -> &mut Self {
        unsafe {
            ffi::multisample_state_set_samples(&mut self.0, samples);
        }
        self
    }

    pub fn set_alpha_to_coverage_enable(&mut self, enable: bool) -> &mut Self {
        unsafe {
            ffi::multisample_state_set_alpha_to_coverage_enable(&mut self.0, enable);
        }
        self
    }

    pub fn set_alpha_to_coverage_dither(&mut self, enable: bool) -> &mut Self {
        unsafe {
            ffi::multisample_state_set_alpha_to_coverage_dither(&mut self.0, enable);
        }
        self
    }

    pub fn set_raster_samples(&mut self, samples: i32) -> &mut Self {
        unsafe {
            ffi::multisample_state_set_raster_samples(&mut self.0, samples);
        }
        self
    }

    pub fn set_coverage_modulation_mode(&mut self, mode: CoverageModulationMode) -> &mut Self {
        unsafe {
            ffi::multisample_state_set_coverage_modulation_mode(&mut self.0, mode);
        }
        self
    }

    pub fn set_coverage_to_color_enable(&mut self, enable: bool) -> &mut Self {
        unsafe {
            ffi::multisample_state_set_coverage_to_color_enable(&mut self.0, enable);
        }
        self
    }

    pub fn set_coverage_to_color_output(&mut self, output: i32) -> &mut Self {
        unsafe {
            ffi::multisample_state_set_coverage_to_color_output(&mut self.0, output);
        }
        self
    }

    pub fn set_sample_locations_enable(&mut self, enable: bool) -> &mut Self {
        unsafe {
            ffi::multisample_state_set_sample_locations_enable(&mut self.0, enable);
        }
        self
    }

    pub fn set_sample_locations_grid_enable(&mut self, enable: bool) -> &mut Self {
        unsafe {
            ffi::multisample_state_set_sample_locations_grid_enable(&mut self.0, enable);
        }
        self
    }

    pub fn set_sample_locations(&mut self, first: i32, count: i32, locations: &[f32]) -> &mut Self {
        unsafe {
            ffi::multisample_state_set_sample_locations(&mut self.0, first, count, locations.as_ptr());
        }
        self
    }

    pub fn get_multisample_enable(&self) -> bool {
        unsafe { ffi::multisample_state_get_multisample_enable(&self.0) }
    }

    pub fn get_samples(&self) -> i32 {
        unsafe { ffi::multisample_state_get_samples(&self.0) }
    }

    pub fn get_alpha_to_coverage_enable(&self) -> bool {
        unsafe { ffi::multisample_state_get_alpha_to_coverage_enable(&self.0) }
    }

    pub fn get_alpha_to_coverage_dither(&self) -> bool {
        unsafe { ffi::multisample_state_get_alpha_to_coverage_dither(&self.0) }
    }

    pub fn get_raster_samples(&self) -> i32 {
        unsafe { ffi::multisample_state_get_raster_samples((&raw const self.0).cast_mut()) }
    }

    pub fn get_coverage_modulation_mode(&self) -> CoverageModulationMode {
        unsafe { ffi::multisample_state_get_coverage_modulation_mode(&self.0) }
    }

    pub fn get_coverage_to_color_enable(&self) -> bool {
        unsafe { ffi::multisample_state_get_coverage_to_color_enable(&self.0) }
    }

    pub fn get_coverage_to_color_output(&self) -> i32 {
        unsafe { ffi::multisample_state_get_coverage_to_color_output(&self.0) }
    }

    pub fn get_sample_locations_enable(&self) -> bool {
        unsafe { ffi::multisample_state_get_sample_locations_enable(&self.0) }
    }

    pub fn get_sample_locations_grid(&self) -> (i32, i32) {
        let (mut w, mut h) = (0, 0);
        unsafe {
            ffi::multisample_state_get_sample_locations_grid((&raw const self.0).cast_mut(), &mut w, &mut h);
        }
        (w, h)
    }

    pub fn get_sample_locations_grid_enable(&self) -> bool {
        unsafe { ffi::multisample_state_get_sample_locations_grid_enable(&self.0) }
    }
}

#[repr(transparent)]
pub struct PolygonState(pub NVNpolygonState);

impl PolygonState {
    pub fn new() -> Self {
        let mut s = PolygonState(NVNpolygonState::default());
        unsafe {
            ffi::polygon_state_set_defaults(&mut s.0);
        }

        s
    }

    pub fn set_cull_face(&mut self, face: Face) -> &mut Self {
        unsafe {
            ffi::polygon_state_set_cull_face(&mut self.0, face);
        }
        self
    }

    pub fn set_front_face(&mut self, face: FrontFace) -> &mut Self {
        unsafe {
            ffi::polygon_state_set_front_face(&mut self.0, face);
        }
        self
    }

    pub fn set_polygon_mode(&mut self, mode: PolygonMode) -> &mut Self {
        unsafe {
            ffi::polygon_state_set_polygon_mode(&mut self.0, mode);
        }
        self
    }

    pub fn set_polygon_offset_enables(&mut self, enables: i32) -> &mut Self {
        unsafe {
            ffi::polygon_state_set_polygon_offset_enables(&mut self.0, enables);
        }
        self
    }

    pub fn get_cull_face(&self) -> Face {
        unsafe { ffi::polygon_state_get_cull_face(&self.0) }
    }

    pub fn get_front_face(&self) -> FrontFace {
        unsafe { ffi::polygon_state_get_front_face(&self.0) }
    }

    pub fn get_polygon_mode(&self) -> PolygonMode {
        unsafe { ffi::polygon_state_get_polygon_mode(&self.0) }
    }

    pub fn get_polygon_offset_enables(&self) -> PolygonOffsetEnable {
        unsafe { ffi::polygon_state_get_polygon_offset_enables(&self.0) }
    }
}

#[repr(transparent)]
pub struct DepthStencilState(pub NVNdepthStencilState);

impl DepthStencilState {
    pub fn new() -> Self {
        let mut s = DepthStencilState(NVNdepthStencilState::default());
        unsafe {
            ffi::depth_stencil_state_set_defaults(&mut s.0);
        }

        s
    }

    pub fn set_depth_test_enable(&mut self, enable: bool) -> &mut Self {
        unsafe {
            ffi::depth_stencil_state_set_depth_test_enable(&mut self.0, enable);
        }
        self
    }

    pub fn set_depth_write_enable(&mut self, enable: bool) -> &mut Self {
        unsafe {
            ffi::depth_stencil_state_set_depth_write_enable(&mut self.0, enable);
        }
        self
    }

    pub fn set_depth_func(&mut self, func: DepthFunc) -> &mut Self {
        unsafe {
            ffi::depth_stencil_state_set_depth_func(&mut self.0, func);
        }
        self
    }

    pub fn set_stencil_test_enable(&mut self, enable: bool) -> &mut Self {
        unsafe {
            ffi::depth_stencil_state_set_stencil_test_enable(&mut self.0, enable);
        }
        self
    }

    pub fn set_stencil_func(&mut self, faces: Face, func: StencilFunc) -> &mut Self {
        unsafe {
            ffi::depth_stencil_state_set_stencil_func(&mut self.0, faces, func);
        }
        self
    }

    pub fn set_stencil_op(&mut self, faces: Face, fail: StencilOp, depth_fail: StencilOp, depth_pass: StencilOp) -> &mut Self {
        unsafe {
            ffi::depth_stencil_state_set_stencil_op(&mut self.0, faces, fail, depth_fail, depth_pass);
        }
        self
    }

    pub fn get_depth_test_enable(&self) -> bool {
        unsafe { ffi::depth_stencil_state_get_depth_test_enable(&self.0) }
    }

    pub fn get_depth_write_enable(&self) -> bool {
        unsafe { ffi::depth_stencil_state_get_depth_write_enable(&self.0) }
    }

    pub fn get_depth_func(&self) -> DepthFunc {
        unsafe { ffi::depth_stencil_state_get_depth_func(&self.0) }
    }

    pub fn get_stencil_test_enable(&self) -> bool {
        unsafe { ffi::depth_stencil_state_get_stencil_test_enable(&self.0) }
    }

    pub fn get_stencil_func(&self, faces: Face) -> StencilFunc {
        unsafe { ffi::depth_stencil_state_get_stencil_func(&self.0, faces) }
    }

    pub fn get_stencil_op(&self, faces: Face) -> (StencilOp, StencilOp, StencilOp) {
        let mut fail = MaybeUninit::uninit();
        let mut depth_fail = MaybeUninit::uninit();
        let mut depth_pass = MaybeUninit::uninit();
        unsafe {
            ffi::depth_stencil_state_get_stencil_op(
                &self.0,
                faces,
                fail.as_mut_ptr(),
                depth_fail.as_mut_ptr(),
                depth_pass.as_mut_ptr(),
            );
            (fail.assume_init(), depth_fail.assume_init(), depth_pass.assume_init())
        }
    }
}

#[repr(transparent)]
pub struct VertexAttribState(pub NVNvertexAttribState);

impl VertexAttribState {
    pub fn new() -> Self {
        let mut s = VertexAttribState(NVNvertexAttribState::default());
        unsafe {
            ffi::vertex_attrib_state_set_defaults(&mut s.0);
        }

        s
    }

    pub fn set_format(&mut self, format: Format, relative_offset: isize) -> &mut Self {
        unsafe {
            ffi::vertex_attrib_state_set_format(&mut self.0, format, relative_offset);
        }
        self
    }

    pub fn set_stream_index(&mut self, stream_index: i32) -> &mut Self {
        unsafe {
            ffi::vertex_attrib_state_set_stream_index(&mut self.0, stream_index);
        }
        self
    }

    pub fn get_format(&self) -> (Format, isize) {
        let mut format = MaybeUninit::uninit();
        let mut offset = 0;
        unsafe {
            ffi::vertex_attrib_state_get_format(&self.0, format.as_mut_ptr(), &mut offset);
            (format.assume_init(), offset)
        }
    }

    pub fn get_stream_index(&self) -> i32 {
        unsafe { ffi::vertex_attrib_state_get_stream_index(&self.0) }
    }
}

#[repr(transparent)]
pub struct VertexStreamState(pub NVNvertexStreamState);

impl VertexStreamState {
    pub fn new() -> Self {
        let mut s = VertexStreamState(NVNvertexStreamState::default());
        unsafe {
            ffi::vertex_stream_state_set_defaults(&mut s.0);
        }

        s
    }

    pub fn set_stride(&mut self, stride: isize) -> &mut Self {
        unsafe {
            ffi::vertex_stream_state_set_stride(&mut self.0, stride);
        }
        self
    }

    pub fn set_divisor(&mut self, divisor: i32) -> &mut Self {
        unsafe {
            ffi::vertex_stream_state_set_divisor(&mut self.0, divisor);
        }
        self
    }

    pub fn get_stride(&self) -> isize {
        unsafe { ffi::vertex_stream_state_get_stride(&self.0) }
    }

    pub fn get_divisor(&self) -> i32 {
        unsafe { ffi::vertex_stream_state_get_divisor(&self.0) }
    }
}
