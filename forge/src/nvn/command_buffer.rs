use core::ffi::{CStr, c_void};

use sys::nvn::{
    fptr::FnCommandBufferMemoryCallback,
    types::{NVNcommandBuffer, NVNtexture, NVNtextureView},
};

use crate::nvn::{
    BarrierBits_, BufferAddress, BufferRange, ClearColorMask_, CommandHandle, ConditionalRenderMode, CopyFlags_, CopyRegion,
    CounterType, DrawPrimitive, EventSignalLocation, EventSignalMode, EventWaitMode, Face, ImageHandle, IndexType,
    SeparateSamplerHandle, SeparateTextureHandle, ShaderStage, ShaderStageBits_, SyncCondition, TextureHandle, TiledCacheAction,
    ViewportSwizzle,
    device::Device,
    event::Event,
    funcs as ffi,
    memory_pool::MemoryPool,
    program::Program,
    sampler_pool::SamplerPool,
    state::{
        BlendState, ChannelMaskState, ColorState, DepthStencilState, MultisampleState, PolygonState, VertexAttribState,
        VertexStreamState,
    },
    sync::Sync,
    texture::{Texture, TextureView, view_ptr},
    texture_pool::TexturePool,
};

#[repr(transparent)]
pub struct CommandBuffer(pub NVNcommandBuffer);

impl CommandBuffer {
    pub fn new(device: &Device) -> Self {
        let mut cmd = NVNcommandBuffer::default();
        unsafe {
            ffi::command_buffer_initialize(&mut cmd, device.as_ptr());
        }

        Self(cmd)
    }

    #[inline]
    pub fn set_debug_label(&mut self, label: &CStr) {
        unsafe {
            ffi::command_buffer_set_debug_label(&mut self.0, label.as_ptr());
        }
    }

    #[inline]
    pub fn set_memory_callback(&mut self, cb: FnCommandBufferMemoryCallback) {
        unsafe {
            ffi::command_buffer_set_memory_callback(&mut self.0, cb);
        }
    }

    #[inline]
    pub fn set_memory_callback_data(&mut self, data: *mut c_void) {
        unsafe {
            ffi::command_buffer_set_memory_callback_data(&mut self.0, data);
        }
    }

    #[inline]
    pub fn get_memory_callback(&self) -> Option<FnCommandBufferMemoryCallback> {
        unsafe { ffi::command_buffer_get_memory_callback(&self.0) }
    }

    #[inline]
    pub fn get_memory_callback_data(&self) -> *mut c_void {
        unsafe { ffi::command_buffer_get_memory_callback_data(&self.0) }
    }

    #[inline]
    pub fn add_command_memory(&mut self, pool: &MemoryPool, offset: isize, size: usize) {
        unsafe {
            ffi::command_buffer_add_command_memory(&mut self.0, &pool.0, offset, size);
        }
    }

    #[inline]
    pub fn add_control_memory(&mut self, memory: *mut c_void, size: usize) {
        unsafe {
            ffi::command_buffer_add_control_memory(&mut self.0, memory, size);
        }
    }

    #[inline]
    pub fn get_command_mem_size(&self) -> usize {
        unsafe { ffi::command_buffer_get_command_memory_size(&self.0) }
    }

    #[inline]
    pub fn get_command_mem_used(&self) -> usize {
        unsafe { ffi::command_buffer_get_command_memory_used(&self.0) }
    }

    #[inline]
    pub fn get_command_mem_free(&self) -> usize {
        unsafe { ffi::command_buffer_get_command_memory_free(&self.0) }
    }

    #[inline]
    pub fn get_control_mem_size(&self) -> usize {
        unsafe { ffi::command_buffer_get_control_memory_size(&self.0) }
    }

    #[inline]
    pub fn get_control_mem_used(&self) -> usize {
        unsafe { ffi::command_buffer_get_control_memory_used(&self.0) }
    }

    #[inline]
    pub fn get_control_mem_free(&self) -> usize {
        unsafe { ffi::command_buffer_get_control_memory_free(&self.0) }
    }

    #[inline]
    pub fn begin_recording(&mut self) {
        unsafe {
            ffi::command_buffer_begin_recording(&mut self.0);
        }
    }

    #[inline]
    pub fn end_recording(&mut self) -> CommandHandle {
        unsafe { ffi::command_buffer_end_recording(&mut self.0) }
    }

    #[inline]
    pub fn is_recording(&self) -> bool {
        unsafe { ffi::command_buffer_is_recording(&self.0) }
    }

    #[inline]
    pub fn call_commands(&mut self, cmds: &[CommandHandle]) {
        unsafe {
            ffi::command_buffer_call_commands(&mut self.0, cmds.len() as i32, cmds.as_ptr());
        }
    }

    #[inline]
    pub fn copy_commands(&mut self, cmds: &[CommandHandle]) {
        unsafe {
            ffi::command_buffer_copy_commands(&mut self.0, cmds.len() as i32, cmds.as_ptr());
        }
    }

    #[inline]
    pub fn bind_blend_state(&mut self, state: &BlendState) {
        unsafe {
            ffi::command_buffer_bind_blend_state(&mut self.0, &state.0);
        }
    }

    #[inline]
    pub fn bind_channel_mask_state(&mut self, state: &ChannelMaskState) {
        unsafe {
            ffi::command_buffer_bind_channel_mask_state(&mut self.0, &state.0);
        }
    }

    #[inline]
    pub fn bind_color_state(&mut self, state: &ColorState) {
        unsafe {
            ffi::command_buffer_bind_color_state(&mut self.0, &state.0);
        }
    }

    #[inline]
    pub fn bind_multisample_state(&mut self, state: &MultisampleState) {
        unsafe {
            ffi::command_buffer_bind_multisample_state(&mut self.0, &state.0);
        }
    }

    #[inline]
    pub fn bind_polygon_state(&mut self, state: &PolygonState) {
        unsafe {
            ffi::command_buffer_bind_polygon_state(&mut self.0, &state.0);
        }
    }

    #[inline]
    pub fn bind_depth_stencil_state(&mut self, state: &DepthStencilState) {
        unsafe {
            ffi::command_buffer_bind_depth_stencil_state(&mut self.0, &state.0);
        }
    }

    #[inline]
    pub fn bind_vertex_attrib_state(&mut self, attribs: &[VertexAttribState]) {
        unsafe {
            ffi::command_buffer_bind_vertex_attrib_state(&mut self.0, attribs.len() as i32, attribs.as_ptr().cast());
        }
    }

    #[inline]
    pub fn bind_vertex_stream_state(&mut self, streams: &[VertexStreamState]) {
        unsafe {
            ffi::command_buffer_bind_vertex_stream_state(&mut self.0, streams.len() as i32, streams.as_ptr().cast());
        }
    }

    #[inline]
    pub fn bind_program(&mut self, program: &Program, stages: ShaderStageBits_) {
        unsafe {
            ffi::command_buffer_bind_program(&mut self.0, &program.0, stages);
        }
    }

    #[inline]
    pub fn bind_vertex_buffer(&mut self, index: i32, buffer: BufferAddress, size: usize) {
        unsafe {
            ffi::command_buffer_bind_vertex_buffer(&mut self.0, index, buffer, size);
        }
    }

    #[inline]
    pub fn bind_vertex_buffers(&mut self, first: i32, buffers: &[BufferRange]) {
        unsafe {
            ffi::command_buffer_bind_vertex_buffers(&mut self.0, first, buffers.len() as i32, buffers.as_ptr());
        }
    }

    #[inline]
    pub fn bind_uniform_buffer(&mut self, stage: ShaderStage, index: i32, buffer: BufferAddress, size: usize) {
        unsafe {
            ffi::command_buffer_bind_uniform_buffer(&mut self.0, stage, index, buffer, size);
        }
    }

    #[inline]
    pub fn bind_uniform_buffers(&mut self, stage: ShaderStage, first: i32, buffers: &[BufferRange]) {
        unsafe {
            ffi::command_buffer_bind_uniform_buffers(&mut self.0, stage, first, buffers.len() as i32, buffers.as_ptr());
        }
    }

    #[inline]
    pub fn bind_transform_feedback_buffer(&mut self, index: i32, buffer: BufferAddress, size: usize) {
        unsafe {
            ffi::command_buffer_bind_transform_feedback_buffer(&mut self.0, index, buffer, size);
        }
    }

    #[inline]
    pub fn bind_transform_feedback_buffers(&mut self, first: i32, buffers: &[BufferRange]) {
        unsafe {
            ffi::command_buffer_bind_transform_feedback_buffers(&mut self.0, first, buffers.len() as i32, buffers.as_ptr());
        }
    }

    #[inline]
    pub fn bind_storage_buffer(&mut self, stage: ShaderStage, index: i32, buffer: BufferAddress, size: usize) {
        unsafe {
            ffi::command_buffer_bind_storage_buffer(&mut self.0, stage, index, buffer, size);
        }
    }

    #[inline]
    pub fn bind_storage_buffers(&mut self, stage: ShaderStage, first: i32, buffers: &[BufferRange]) {
        unsafe {
            ffi::command_buffer_bind_storage_buffers(&mut self.0, stage, first, buffers.len() as i32, buffers.as_ptr());
        }
    }

    #[inline]
    pub fn bind_texture(&mut self, stage: ShaderStage, index: i32, texture: TextureHandle) {
        unsafe {
            ffi::command_buffer_bind_texture(&mut self.0, stage, index, texture);
        }
    }

    #[inline]
    pub fn bind_textures(&mut self, stage: ShaderStage, first: i32, textures: &[TextureHandle]) {
        unsafe {
            ffi::command_buffer_bind_textures(&mut self.0, stage, first, textures.len() as i32, textures.as_ptr());
        }
    }

    #[inline]
    pub fn bind_image(&mut self, stage: ShaderStage, index: i32, image: ImageHandle) {
        unsafe {
            ffi::command_buffer_bind_image(&mut self.0, stage, index, image);
        }
    }

    #[inline]
    pub fn bind_images(&mut self, stage: ShaderStage, first: i32, images: &[ImageHandle]) {
        unsafe {
            ffi::command_buffer_bind_images(&mut self.0, stage, first, images.len() as i32, images.as_ptr());
        }
    }

    #[inline]
    pub fn bind_separate_texture(&mut self, stage: ShaderStage, index: i32, handle: SeparateTextureHandle) {
        unsafe {
            ffi::command_buffer_bind_separate_texture(&mut self.0, stage, index, handle);
        }
    }

    #[inline]
    pub fn bind_separate_sampler(&mut self, stage: ShaderStage, index: i32, handle: SeparateSamplerHandle) {
        unsafe {
            ffi::command_buffer_bind_separate_sampler(&mut self.0, stage, index, handle);
        }
    }

    #[inline]
    pub fn bind_separate_textures(&mut self, stage: ShaderStage, first: i32, handles: &[SeparateTextureHandle]) {
        unsafe {
            ffi::command_buffer_bind_separate_textures(&mut self.0, stage, first, handles.len() as i32, handles.as_ptr());
        }
    }

    #[inline]
    pub fn bind_separate_samplers(&mut self, stage: ShaderStage, first: i32, handles: &[SeparateSamplerHandle]) {
        unsafe {
            ffi::command_buffer_bind_separate_samplers(&mut self.0, stage, first, handles.len() as i32, handles.as_ptr());
        }
    }

    #[inline]
    pub fn bind_coverage_modulation_table(&mut self, table: &[f32]) {
        unsafe {
            ffi::command_buffer_bind_coverage_modulation_table(&mut self.0, table.as_ptr());
        }
    }

    #[inline]
    pub fn set_patch_size(&mut self, size: i32) {
        unsafe {
            ffi::command_buffer_set_patch_size(&mut self.0, size);
        }
    }

    #[inline]
    pub fn set_inner_tessellation_levels(&mut self, levels: &[f32; 2]) {
        unsafe {
            ffi::command_buffer_set_inner_tessellation_levels(&mut self.0, levels.as_ptr());
        }
    }

    #[inline]
    pub fn set_outer_tessellation_levels(&mut self, levels: &[f32; 4]) {
        unsafe {
            ffi::command_buffer_set_outer_tessellation_levels(&mut self.0, levels.as_ptr());
        }
    }

    #[inline]
    pub fn set_primitive_restart(&mut self, enable: bool, index: i32) {
        unsafe {
            ffi::command_buffer_set_primitive_restart(&mut self.0, enable, index);
        }
    }

    #[inline]
    pub fn begin_transform_feedback(&mut self, buffer: BufferAddress) {
        unsafe {
            ffi::command_buffer_begin_transform_feedback(&mut self.0, buffer);
        }
    }

    #[inline]
    pub fn end_transform_feedback(&mut self, buffer: BufferAddress) {
        unsafe {
            ffi::command_buffer_end_transform_feedback(&mut self.0, buffer);
        }
    }

    #[inline]
    pub fn pause_transform_feedback(&mut self, buffer: BufferAddress) {
        unsafe {
            ffi::command_buffer_pause_transform_feedback(&mut self.0, buffer);
        }
    }

    #[inline]
    pub fn resume_transform_feedback(&mut self, buffer: BufferAddress) {
        unsafe {
            ffi::command_buffer_resume_transform_feedback(&mut self.0, buffer);
        }
    }

    #[inline]
    pub fn draw_transform_feedback(&mut self, mode: DrawPrimitive, buffer: BufferAddress) {
        unsafe {
            ffi::command_buffer_draw_transform_feedback(&mut self.0, mode, buffer);
        }
    }

    #[inline]
    pub fn draw_arrays(&mut self, mode: DrawPrimitive, first: i32, count: i32) {
        unsafe {
            ffi::command_buffer_draw_arrays(&mut self.0, mode, first, count);
        }
    }

    #[inline]
    pub fn draw_elements(&mut self, mode: DrawPrimitive, ty: IndexType, count: i32, index_buffer: BufferAddress) {
        unsafe {
            ffi::command_buffer_draw_elements(&mut self.0, mode, ty, count, index_buffer);
        }
    }

    #[inline]
    pub fn draw_elements_base_vertex(
        &mut self,
        mode: DrawPrimitive,
        ty: IndexType,
        count: i32,
        index_buffer: BufferAddress,
        base_vertex: i32,
    ) {
        unsafe {
            ffi::command_buffer_draw_elements_base_vertex(&mut self.0, mode, ty, count, index_buffer, base_vertex);
        }
    }

    #[inline]
    pub fn draw_arrays_instanced(
        &mut self,
        mode: DrawPrimitive,
        first: i32,
        count: i32,
        base_instance: i32,
        instance_count: i32,
    ) {
        unsafe {
            ffi::command_buffer_draw_arrays_instanced(&mut self.0, mode, first, count, base_instance, instance_count);
        }
    }

    #[inline]
    pub fn draw_elements_instanced(
        &mut self,
        mode: DrawPrimitive,
        ty: IndexType,
        count: i32,
        index_buffer: BufferAddress,
        base_vertex: i32,
        base_instance: i32,
        instance_count: i32,
    ) {
        unsafe {
            ffi::command_buffer_draw_elements_instanced(
                &mut self.0,
                mode,
                ty,
                count,
                index_buffer,
                base_vertex,
                base_instance,
                instance_count,
            );
        }
    }

    #[inline]
    pub fn draw_arrays_indirect(&mut self, mode: DrawPrimitive, indirect_buffer: BufferAddress) {
        unsafe {
            ffi::command_buffer_draw_arrays_indirect(&mut self.0, mode, indirect_buffer);
        }
    }

    #[inline]
    pub fn draw_elements_indirect(
        &mut self,
        mode: DrawPrimitive,
        ty: IndexType,
        index_buffer: BufferAddress,
        indirect_buffer: BufferAddress,
    ) {
        unsafe {
            ffi::command_buffer_draw_elements_indirect(&mut self.0, mode, ty, index_buffer, indirect_buffer);
        }
    }

    #[inline]
    pub fn multi_draw_arrays_indirect_count(
        &mut self,
        mode: DrawPrimitive,
        indirect_buffer: BufferAddress,
        parameter_buffer: BufferAddress,
        max_draw_count: i32,
        stride: isize,
    ) {
        unsafe {
            ffi::command_buffer_multi_draw_arrays_indirect_count(
                &mut self.0,
                mode,
                indirect_buffer,
                parameter_buffer,
                max_draw_count,
                stride,
            );
        }
    }

    #[inline]
    pub fn multi_draw_elements_indirect_count(
        &mut self,
        mode: DrawPrimitive,
        ty: IndexType,
        index_buffer: BufferAddress,
        indirect_buffer: BufferAddress,
        parameter_buffer: BufferAddress,
        max_draw_count: i32,
        stride: isize,
    ) {
        unsafe {
            ffi::command_buffer_multi_draw_elements_indirect_count(
                &mut self.0,
                mode,
                ty,
                index_buffer,
                indirect_buffer,
                parameter_buffer,
                max_draw_count,
                stride,
            );
        }
    }

    #[inline]
    pub fn clear_color(&mut self, index: i32, color: &[f32; 4], mask: ClearColorMask_) {
        unsafe {
            ffi::command_buffer_clear_color(&mut self.0, index, color.as_ptr(), mask);
        }
    }

    #[inline]
    pub fn clear_color_i(&mut self, index: i32, color: &[i32; 4], mask: ClearColorMask_) {
        unsafe {
            ffi::command_buffer_clear_colori(&mut self.0, index, color.as_ptr(), mask);
        }
    }

    #[inline]
    pub fn clear_color_ui(&mut self, index: i32, color: &[u32; 4], mask: ClearColorMask_) {
        unsafe {
            ffi::command_buffer_clear_colorui(&mut self.0, index, color.as_ptr(), mask);
        }
    }

    #[inline]
    pub fn clear_depth_stencil(&mut self, depth_value: f32, depth_mask: bool, stencil_value: i32, stencil_mask: i32) {
        unsafe {
            ffi::command_buffer_clear_depth_stencil(&mut self.0, depth_value, depth_mask, stencil_value, stencil_mask);
        }
    }

    #[inline]
    pub fn dispatch_compute(&mut self, groups_x: i32, groups_y: i32, groups_z: i32) {
        unsafe {
            ffi::command_buffer_dispatch_compute(&mut self.0, groups_x, groups_y, groups_z);
        }
    }

    #[inline]
    pub fn dispatch_compute_indirect(&mut self, buffer: BufferAddress) {
        unsafe {
            ffi::command_buffer_dispatch_compute_indirect(&mut self.0, buffer);
        }
    }

    #[inline]
    pub fn set_viewport(&mut self, x: i32, y: i32, w: i32, h: i32) {
        unsafe {
            ffi::command_buffer_set_viewport(&mut self.0, x, y, w, h);
        }
    }

    /// Each viewport is `[x, y, w, h]`.
    #[inline]
    pub fn set_viewports(&mut self, first: i32, viewports: &[[f32; 4]]) {
        unsafe {
            ffi::command_buffer_set_viewports(&mut self.0, first, viewports.len() as i32, viewports.as_ptr().cast());
        }
    }

    #[inline]
    pub fn set_viewport_swizzles(&mut self, first: i32, count: i32, swizzles: &[ViewportSwizzle]) {
        unsafe {
            ffi::command_buffer_set_viewport_swizzles(&mut self.0, first, count, swizzles.as_ptr());
        }
    }

    #[inline]
    pub fn set_scissor(&mut self, x: i32, y: i32, w: i32, h: i32) {
        unsafe {
            ffi::command_buffer_set_scissor(&mut self.0, x, y, w, h);
        }
    }

    /// Each scissor is `[x, y, w, h]`.
    #[inline]
    pub fn set_scissors(&mut self, first: i32, scissors: &[[i32; 4]]) {
        unsafe {
            ffi::command_buffer_set_scissors(&mut self.0, first, scissors.len() as i32, scissors.as_ptr().cast());
        }
    }

    #[inline]
    pub fn set_depth_range(&mut self, near: f32, far: f32) {
        unsafe {
            ffi::command_buffer_set_depth_range(&mut self.0, near, far);
        }
    }

    #[inline]
    pub fn set_depth_bounds(&mut self, enable: bool, near: f32, far: f32) {
        unsafe {
            ffi::command_buffer_set_depth_bounds(&mut self.0, enable, near, far);
        }
    }

    /// Each range is `[near, far]`.
    #[inline]
    pub fn set_depth_ranges(&mut self, first: i32, ranges: &[[f32; 2]]) {
        unsafe {
            ffi::command_buffer_set_depth_ranges(&mut self.0, first, ranges.len() as i32, ranges.as_ptr().cast());
        }
    }

    #[inline]
    pub fn set_tiled_cache_action(&mut self, action: TiledCacheAction) {
        unsafe {
            ffi::command_buffer_set_tiled_cache_action(&mut self.0, action);
        }
    }

    #[inline]
    pub fn set_tiled_cache_tile_size(&mut self, w: i32, h: i32) {
        unsafe {
            ffi::command_buffer_set_tiled_cache_tile_size(&mut self.0, w, h);
        }
    }

    #[inline]
    pub fn set_stencil_value_mask(&mut self, faces: Face, mask: i32) {
        unsafe {
            ffi::command_buffer_set_stencil_value_mask(&mut self.0, faces, mask);
        }
    }

    #[inline]
    pub fn set_stencil_mask(&mut self, faces: Face, mask: i32) {
        unsafe {
            ffi::command_buffer_set_stencil_mask(&mut self.0, faces, mask);
        }
    }

    #[inline]
    pub fn set_stencil_ref(&mut self, faces: Face, value: i32) {
        unsafe {
            ffi::command_buffer_set_stencil_ref(&mut self.0, faces, value);
        }
    }

    #[inline]
    pub fn set_blend_color(&mut self, color: &[f32; 4]) {
        unsafe {
            ffi::command_buffer_set_blend_color(&mut self.0, color.as_ptr());
        }
    }

    #[inline]
    pub fn set_point_size(&mut self, size: f32) {
        unsafe {
            ffi::command_buffer_set_point_size(&mut self.0, size);
        }
    }

    #[inline]
    pub fn set_line_width(&mut self, width: f32) {
        unsafe {
            ffi::command_buffer_set_line_width(&mut self.0, width);
        }
    }

    #[inline]
    pub fn set_polygon_offset_clamp(&mut self, factor: f32, units: f32, clamp: f32) {
        unsafe {
            ffi::command_buffer_set_polygon_offset_clamp(&mut self.0, factor, units, clamp);
        }
    }

    #[inline]
    pub fn set_alpha_ref(&mut self, value: f32) {
        unsafe {
            ffi::command_buffer_set_alpha_ref(&mut self.0, value);
        }
    }

    #[inline]
    pub fn set_sample_mask(&mut self, mask: i32) {
        unsafe {
            ffi::command_buffer_set_sample_mask(&mut self.0, mask);
        }
    }

    #[inline]
    pub fn set_rasterizer_discard(&mut self, discard: bool) {
        unsafe {
            ffi::command_buffer_set_rasterizer_discard(&mut self.0, discard);
        }
    }

    #[inline]
    pub fn set_depth_clamp(&mut self, clamp: bool) {
        unsafe {
            ffi::command_buffer_set_depth_clamp(&mut self.0, clamp);
        }
    }

    #[inline]
    pub fn set_conservative_raster_enable(&mut self, enable: bool) {
        unsafe {
            ffi::command_buffer_set_conservative_raster_enable(&mut self.0, enable);
        }
    }

    #[inline]
    pub fn set_conservative_raster_dilate(&mut self, dilate: f32) {
        unsafe {
            ffi::command_buffer_set_conservative_raster_dilate(&mut self.0, dilate);
        }
    }

    #[inline]
    pub fn set_subpixel_precision_bias(&mut self, x_bits: i32, y_bits: i32) {
        unsafe {
            ffi::command_buffer_set_subpixel_precision_bias(&mut self.0, x_bits, y_bits);
        }
    }

    #[inline]
    pub fn copy_buffer_to_texture(
        &mut self,
        buffer: BufferAddress,
        dst: &Texture,
        dst_view: Option<&TextureView>,
        dst_region: &CopyRegion,
        flags: CopyFlags_,
    ) {
        unsafe {
            ffi::command_buffer_copy_buffer_to_texture(&mut self.0, buffer, &dst.0, view_ptr(dst_view), dst_region, flags);
        }
    }

    #[inline]
    pub fn copy_texture_to_buffer(
        &mut self,
        src: &Texture,
        src_view: Option<&TextureView>,
        src_region: &CopyRegion,
        buffer: BufferAddress,
        flags: CopyFlags_,
    ) {
        unsafe {
            ffi::command_buffer_copy_texture_to_buffer(&mut self.0, &src.0, view_ptr(src_view), src_region, buffer, flags);
        }
    }

    #[inline]
    pub fn copy_texture_to_texture(
        &mut self,
        src: &Texture,
        src_view: Option<&TextureView>,
        src_region: &CopyRegion,
        dst: &Texture,
        dst_view: Option<&TextureView>,
        dst_region: &CopyRegion,
        flags: CopyFlags_,
    ) {
        unsafe {
            ffi::command_buffer_copy_texture_to_texture(
                &mut self.0,
                &src.0,
                view_ptr(src_view),
                src_region,
                &dst.0,
                view_ptr(dst_view),
                dst_region,
                flags,
            );
        }
    }

    #[inline]
    pub fn copy_buffer_to_buffer(&mut self, src: BufferAddress, dst: BufferAddress, size: usize, flags: CopyFlags_) {
        unsafe {
            ffi::command_buffer_copy_buffer_to_buffer(&mut self.0, src, dst, size, flags);
        }
    }

    #[inline]
    pub fn clear_buffer(&mut self, buffer: BufferAddress, size: usize, value: u32) {
        unsafe {
            ffi::command_buffer_clear_buffer(&mut self.0, buffer, size, value);
        }
    }

    #[inline]
    pub fn clear_texture(
        &mut self,
        dst: &Texture,
        dst_view: Option<&TextureView>,
        dst_region: &CopyRegion,
        color: &[f32; 4],
        mask: ClearColorMask_,
    ) {
        unsafe {
            ffi::command_buffer_clear_texture(&mut self.0, &dst.0, view_ptr(dst_view), dst_region, color.as_ptr(), mask);
        }
    }

    #[inline]
    pub fn clear_texture_i(
        &mut self,
        dst: &Texture,
        dst_view: Option<&TextureView>,
        dst_region: &CopyRegion,
        color: &[i32; 4],
        mask: ClearColorMask_,
    ) {
        unsafe {
            ffi::command_buffer_clear_texturei(&mut self.0, &dst.0, view_ptr(dst_view), dst_region, color.as_ptr(), mask);
        }
    }

    #[inline]
    pub fn clear_texture_ui(
        &mut self,
        dst: &Texture,
        dst_view: Option<&TextureView>,
        dst_region: &CopyRegion,
        color: &[u32; 4],
        mask: ClearColorMask_,
    ) {
        unsafe {
            ffi::command_buffer_clear_textureui(&mut self.0, &dst.0, view_ptr(dst_view), dst_region, color.as_ptr(), mask);
        }
    }

    #[inline]
    pub fn update_uniform_buffer(&mut self, buffer: BufferAddress, buffer_size: usize, offset: isize, data: &[u8]) {
        unsafe {
            ffi::command_buffer_update_uniform_buffer(&mut self.0, buffer, buffer_size, offset, data.len(), data.as_ptr().cast());
        }
    }

    #[inline]
    pub fn report_counter(&mut self, counter: CounterType, buffer: BufferAddress) {
        unsafe {
            ffi::command_buffer_report_counter(&mut self.0, counter, buffer);
        }
    }

    #[inline]
    pub fn reset_counter(&mut self, counter: CounterType) {
        unsafe {
            ffi::command_buffer_reset_counter(&mut self.0, counter);
        }
    }

    #[inline]
    pub fn report_value(&mut self, value: u32, buffer: BufferAddress) {
        unsafe {
            ffi::command_buffer_report_value(&mut self.0, value, buffer);
        }
    }

    #[inline]
    pub fn set_render_enable(&mut self, enable: bool) {
        unsafe {
            ffi::command_buffer_set_render_enable(&mut self.0, enable);
        }
    }

    #[inline]
    pub fn set_render_enable_conditional(&mut self, mode: ConditionalRenderMode, counter: BufferAddress) {
        unsafe {
            ffi::command_buffer_set_render_enable_conditional(&mut self.0, mode, counter);
        }
    }

    /// # Panics
    /// Panics if `color_views` is given and its length differs from `colors`.
    #[inline]
    pub fn set_render_targets(
        &mut self,
        colors: &[&Texture],
        color_views: Option<&[Option<&TextureView>]>,
        depth_stencil: Option<&Texture>,
        depth_stencil_view: Option<&TextureView>,
    ) {
        let color_views = match color_views {
            Some(views) => {
                assert_eq!(views.len(), colors.len(), "color_views must match colors in length");
                // `Option<&TextureView>` has the same layout as `*const NVNtextureView`
                views.as_ptr().cast::<*const NVNtextureView>()
            }
            None => core::ptr::null(),
        };
        let depth_stencil = depth_stencil.map_or(core::ptr::null(), |t| &raw const t.0);

        unsafe {
            ffi::command_buffer_set_render_targets(
                &mut self.0,
                colors.len() as i32,
                colors.as_ptr().cast::<*const NVNtexture>(),
                color_views,
                depth_stencil,
                view_ptr(depth_stencil_view),
            );
        }
    }

    #[inline]
    pub fn discard_color(&mut self, index: i32) {
        unsafe {
            ffi::command_buffer_discard_color(&mut self.0, index);
        }
    }

    #[inline]
    pub fn discard_depth_stencil(&mut self) {
        unsafe {
            ffi::command_buffer_discard_depth_stencil(&mut self.0);
        }
    }

    #[inline]
    pub fn downsample(&mut self, src: &Texture, dst: &Texture) {
        unsafe {
            ffi::command_buffer_downsample(&mut self.0, &src.0, &dst.0);
        }
    }

    #[inline]
    pub fn tiled_downsample(&mut self, src: &Texture, dst: &Texture) {
        unsafe {
            ffi::command_buffer_tiled_downsample(&mut self.0, &src.0, &dst.0);
        }
    }

    #[inline]
    pub fn downsample_texture_view(
        &mut self,
        src: &Texture,
        src_view: Option<&TextureView>,
        dst: &Texture,
        dst_view: Option<&TextureView>,
    ) {
        unsafe {
            ffi::command_buffer_downsample_texture_view(&mut self.0, &src.0, view_ptr(src_view), &dst.0, view_ptr(dst_view));
        }
    }

    #[inline]
    pub fn tiled_downsample_texture_view(
        &mut self,
        src: &Texture,
        src_view: Option<&TextureView>,
        dst: &Texture,
        dst_view: Option<&TextureView>,
    ) {
        unsafe {
            ffi::command_buffer_tiled_downsample_texture_view(
                &mut self.0,
                &src.0,
                view_ptr(src_view),
                &dst.0,
                view_ptr(dst_view),
            );
        }
    }

    #[inline]
    pub fn barrier(&mut self, barrier: BarrierBits_) {
        unsafe {
            ffi::command_buffer_barrier(&mut self.0, barrier);
        }
    }

    #[inline]
    pub fn wait_sync(&mut self, sync: &Sync) {
        unsafe {
            ffi::command_buffer_wait_sync(&mut self.0, &sync.0);
        }
    }

    #[inline]
    pub fn fence_sync(&mut self, sync: &mut Sync, cond: SyncCondition, flush_cpu: bool) {
        unsafe {
            ffi::command_buffer_fence_sync(&mut self.0, &mut sync.0, cond, flush_cpu as i32);
        }
    }

    #[inline]
    pub fn wait_event(&mut self, event: &Event, mode: EventWaitMode, value: u32) {
        unsafe {
            ffi::command_buffer_wait_event(&mut self.0, &event.0, mode, value);
        }
    }

    #[inline]
    pub fn signal_event(&mut self, event: &Event, mode: EventSignalMode, location: EventSignalLocation, flags: i32, value: u32) {
        unsafe {
            ffi::command_buffer_signal_event(&mut self.0, &event.0, mode, location, flags, value);
        }
    }

    #[inline]
    pub fn set_texture_pool(&mut self, pool: &TexturePool) {
        unsafe {
            ffi::command_buffer_set_texture_pool(&mut self.0, &pool.0);
        }
    }

    #[inline]
    pub fn set_sampler_pool(&mut self, pool: &SamplerPool) {
        unsafe {
            ffi::command_buffer_set_sampler_pool(&mut self.0, &pool.0);
        }
    }

    #[inline]
    pub fn set_shader_scratch_memory(&mut self, pool: &MemoryPool, offset: isize, size: usize) {
        unsafe {
            ffi::command_buffer_set_shader_scratch_memory(&mut self.0, &pool.0, offset, size);
        }
    }

    #[inline]
    pub fn save_zcull_data(&mut self, addr: BufferAddress, size: usize) {
        unsafe {
            ffi::command_buffer_save_zcull_data(&mut self.0, addr, size);
        }
    }

    #[inline]
    pub fn restore_zcull_data(&mut self, addr: BufferAddress, size: usize) {
        unsafe {
            ffi::command_buffer_restore_zcull_data(&mut self.0, addr, size);
        }
    }

    #[inline]
    pub fn set_copy_row_stride(&mut self, stride: isize) {
        unsafe {
            ffi::command_buffer_set_copy_row_stride(&mut self.0, stride);
        }
    }

    #[inline]
    pub fn set_copy_image_stride(&mut self, stride: isize) {
        unsafe {
            ffi::command_buffer_set_copy_image_stride(&mut self.0, stride);
        }
    }

    #[inline]
    pub fn get_copy_row_stride(&self) -> isize {
        unsafe { ffi::command_buffer_get_copy_row_stride(&self.0) }
    }

    #[inline]
    pub fn get_copy_image_stride(&self) -> isize {
        unsafe { ffi::command_buffer_get_copy_image_stride(&self.0) }
    }

    #[inline]
    pub fn set_program_subroutines(&mut self, program: &mut Program, stage: ShaderStage, first: i32, values: &[i32]) {
        unsafe {
            ffi::command_buffer_set_program_subroutines(
                &mut self.0,
                &mut program.0,
                stage,
                first,
                values.len() as i32,
                values.as_ptr(),
            );
        }
    }

    #[inline]
    pub fn resolve_depth_buffer(&mut self) {
        unsafe {
            ffi::command_buffer_resolve_depth_buffer(&mut self.0);
        }
    }

    #[inline]
    pub fn push_debug_group_static(&mut self, domain_id: u32, description: &CStr) {
        unsafe {
            ffi::command_buffer_push_debug_group_static(&mut self.0, domain_id, description.as_ptr());
        }
    }

    #[inline]
    pub fn push_debug_group_dynamic(&mut self, domain_id: u32, description: &CStr) {
        unsafe {
            ffi::command_buffer_push_debug_group_dynamic(&mut self.0, domain_id, description.as_ptr());
        }
    }

    #[inline]
    pub fn push_debug_group(&mut self, domain_id: u32, description: &CStr) {
        unsafe {
            ffi::command_buffer_push_debug_group(&mut self.0, domain_id, description.as_ptr());
        }
    }

    #[inline]
    pub fn pop_debug_group(&mut self) {
        unsafe {
            ffi::command_buffer_pop_debug_group(&mut self.0);
        }
    }

    #[inline]
    pub fn pop_debug_group_id(&mut self, domain_id: u32) {
        unsafe {
            ffi::command_buffer_pop_debug_group_id(&mut self.0, domain_id);
        }
    }

    #[inline]
    pub fn insert_debug_marker_static(&mut self, domain_id: u32, description: &CStr) {
        unsafe {
            ffi::command_buffer_insert_debug_marker_static(&mut self.0, domain_id, description.as_ptr());
        }
    }

    #[inline]
    pub fn insert_debug_marker_dynamic(&mut self, domain_id: u32, description: &CStr) {
        unsafe {
            ffi::command_buffer_insert_debug_marker_dynamic(&mut self.0, domain_id, description.as_ptr());
        }
    }

    #[inline]
    pub fn insert_debug_marker(&mut self, description: &CStr) {
        unsafe {
            ffi::command_buffer_insert_debug_marker(&mut self.0, description.as_ptr());
        }
    }
}

impl Drop for CommandBuffer {
    fn drop(&mut self) {
        unsafe {
            ffi::command_buffer_finalize(&mut self.0);
        }
    }
}
