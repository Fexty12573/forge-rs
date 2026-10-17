use core::ffi::{CStr, c_uchar, c_void};

use sys::nvn::types::NVNdevice;

use crate::nvn::{
    CommandHandle, CounterData, DebugCallbackSeverity, DebugCallbackSource, DebugCallbackType, DebugDomainId, DepthMode,
    DeviceInfo, Format, SeparateSamplerHandle, SeparateTextureHandle, WindowOriginMode,
    texture::{ImageHandle, TextureHandle},
};

use super::funcs as ffi;

pub struct Device(pub *mut NVNdevice);

impl Device {
    pub fn from_ptr(ptr: *mut NVNdevice) -> Self {
        Self(ptr)
    }

    #[inline]
    pub fn finalize(&mut self) {
        unsafe {
            ffi::device_finalize(self.0);
        }
    }

    #[inline]
    pub fn set_debug_label(&mut self, label: &CStr) {
        unsafe {
            ffi::device_set_debug_label(self.0, label.as_ptr());
        }
    }

    #[inline]
    pub fn get_integer(&self, info: DeviceInfo) -> i32 {
        let mut int = 0;
        unsafe {
            ffi::device_get_integer(self.0, info, &mut int);
        }

        int
    }

    #[inline]
    pub fn get_current_timestamp_in_ns(&self) -> u64 {
        unsafe { ffi::device_get_current_timestamp_in_nanoseconds(self.0) }
    }

    #[inline]
    pub fn get_timestamp_in_ns(&self, mut data: CounterData) -> u64 {
        unsafe { ffi::device_get_timestamp_in_nanoseconds(self.0, &mut data) }
    }

    #[inline]
    pub fn set_intermediate_shader_cache(&mut self, max_entries: i32) {
        unsafe {
            ffi::device_set_intermediate_shader_cache(self.0, max_entries);
        }
    }

    #[inline]
    pub fn get_texture_handle(&self, tex_id: i32, samp_id: i32) -> TextureHandle {
        unsafe { ffi::device_get_texture_handle(self.0, tex_id, samp_id) }
    }

    #[inline]
    pub fn get_texel_fetch_handle(&self, tex_id: i32) -> TextureHandle {
        unsafe { ffi::device_get_texel_fetch_handle(self.0, tex_id) }
    }

    #[inline]
    pub fn get_image_handle(&self, img_id: i32) -> ImageHandle {
        unsafe { ffi::device_get_image_handle(self.0, img_id) }
    }

    #[inline]
    pub fn install_debug_callback(&mut self, cb: DebugCallback) {
        unsafe { ffi::device_install_debug_callback(self.0, debug_trampoline, cb as *mut c_void, true) }
    }

    #[inline]
    pub fn remove_debug_callback(&mut self, cb: DebugCallback) {
        unsafe { ffi::device_install_debug_callback(self.0, debug_trampoline, cb as *mut c_void, false) }
    }

    #[inline]
    pub fn generate_debug_domain_id(&self, name: &CStr) -> DebugDomainId {
        unsafe { ffi::device_generate_debug_domain_id(self.0, name.as_ptr()) }
    }

    #[inline]
    pub fn set_window_origin_mode(&mut self, mode: WindowOriginMode) {
        unsafe {
            ffi::device_set_window_origin_mode(self.0, mode);
        }
    }

    #[inline]
    pub fn get_window_origin_mode(&self) -> WindowOriginMode {
        unsafe { ffi::device_get_window_origin_mode(self.0) }
    }

    #[inline]
    pub fn set_depth_mode(&mut self, mode: DepthMode) {
        unsafe {
            ffi::device_set_depth_mode(self.0, mode);
        }
    }

    #[inline]
    pub fn get_depth_mode(&self) -> DepthMode {
        unsafe { ffi::device_get_depth_mode(self.0) }
    }

    #[inline]
    pub fn register_fast_clear_color(&mut self, color: &[f32], format: Format) -> bool {
        unsafe { ffi::device_register_fast_clear_color(self.0, color.as_ptr(), format) }
    }

    #[inline]
    pub fn register_fast_clear_color_i(&mut self, color: &[i32], format: Format) -> bool {
        unsafe { ffi::device_register_fast_clear_colori(self.0, color.as_ptr(), format) }
    }

    #[inline]
    pub fn register_fast_clear_color_ui(&mut self, color: &[u32], format: Format) -> bool {
        unsafe { ffi::device_register_fast_clear_colorui(self.0, color.as_ptr(), format) }
    }

    #[inline]
    pub fn register_fast_clear_depth(&mut self, depth: f32) -> bool {
        unsafe { ffi::device_register_fast_clear_depth(self.0, depth) }
    }

    #[inline]
    pub fn apply_deferred_finalizes(&mut self, i: i32) {
        unsafe {
            ffi::device_apply_deferred_finalizes(self.0, i);
        }
    }

    #[inline]
    pub fn finalize_command_handle(&mut self, handle: CommandHandle) {
        unsafe {
            ffi::device_finalize_command_handle(self.0, handle);
        }
    }

    #[inline]
    pub fn get_separate_texture_handle(&self, tex_id: i32) -> SeparateTextureHandle {
        unsafe { ffi::device_get_separate_texture_handle(self.0, tex_id) }
    }

    #[inline]
    pub fn get_separate_sampler_handle(&self, samp_id: i32) -> SeparateSamplerHandle {
        unsafe { ffi::device_get_separate_sampler_handle(self.0, samp_id) }
    }

    #[inline]
    pub fn is_external_debugger_attached(&self) -> bool {
        unsafe { ffi::device_is_external_debugger_attached(self.0) }
    }
}

pub struct DebugMessage<'a> {
    pub source: DebugCallbackSource,
    pub ty: DebugCallbackType,
    pub id: i32,
    pub severity: DebugCallbackSeverity,
    pub message: &'a CStr,
}

pub type DebugCallback = fn(&DebugMessage);

unsafe extern "C" fn debug_trampoline(
    source: DebugCallbackSource,
    ty: DebugCallbackType,
    id: i32,
    severity: DebugCallbackSeverity,
    message: *const c_uchar,
    user: *const c_void,
) {
    // `user` is the `DebugCallback` we passed to nvnDeviceInstallDebugCallback
    let cb = unsafe { core::mem::transmute::<*const c_void, DebugCallback>(user) };
    let message = if message.is_null() {
        c""
    } else {
        unsafe { CStr::from_ptr(message.cast()) }
    };
    cb(&DebugMessage {
        source,
        ty,
        id,
        severity,
        message,
    });
}
