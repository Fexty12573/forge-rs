mod builder;

use core::ffi::CStr;

pub use builder::QueueBuilder;
use sys::nvn::types::{NVNqueue, NVNsync, NVNwindow};

use crate::nvn::{CommandHandle, QueueAcquireTextureResult, QueueErrorInfo, QueueGetErrorResult, SyncCondition, funcs as ffi};

// TODO: replace NVN* types with proper counterparts

pub struct Queue(pub *mut NVNqueue);

impl Queue {
    pub fn from_ptr(ptr: *mut NVNqueue) -> Self {
        Self(ptr)
    }

    #[inline]
    pub fn initialize(&mut self, builder: &QueueBuilder) -> bool {
        unsafe { ffi::queue_initialize(self.0, &builder.0) }
    }

    #[inline]
    pub fn finalize(&mut self) {
        unsafe {
            ffi::queue_finalize(self.0);
        }
    }

    #[inline]
    pub fn set_debug_label(&mut self, label: &CStr) {
        unsafe {
            ffi::queue_set_debug_label(self.0, label.as_ptr());
        }
    }

    #[inline]
    pub fn total_command_mem_used(&self) -> usize {
        unsafe { ffi::queue_get_total_command_memory_used(self.0) }
    }

    #[inline]
    pub fn total_control_mem_used(&self) -> usize {
        unsafe { ffi::queue_get_total_control_memory_used(self.0) }
    }

    #[inline]
    pub fn total_compute_mem_used(&self) -> usize {
        unsafe { ffi::queue_get_total_compute_memory_used(self.0) }
    }

    #[inline]
    pub fn get_error(&self, info: *mut QueueErrorInfo) -> QueueGetErrorResult {
        unsafe { ffi::queue_get_error(self.0, info) }
    }

    #[inline]
    pub fn reset_mem_usage_counts(&mut self) {
        unsafe {
            ffi::queue_reset_memory_usage_counts(self.0);
        }
    }

    #[inline]
    pub fn submit(&mut self, cmds: &[CommandHandle]) {
        unsafe {
            ffi::queue_submit_commands(self.0, cmds.len() as i32, cmds.as_ptr());
        }
    }

    #[inline]
    pub fn flush(&mut self) {
        unsafe {
            ffi::queue_flush(self.0);
        }
    }

    #[inline]
    pub fn finish(&mut self) {
        unsafe {
            ffi::queue_finish(self.0);
        }
    }

    #[inline]
    pub fn present_texture(&mut self, window: &mut NVNwindow, tex_id: i32) {
        unsafe {
            ffi::queue_present_texture(self.0, &mut *window, tex_id);
        }
    }

    #[inline]
    pub fn acquire_texture(&mut self, window: &mut NVNwindow) -> QueueAcquireTextureResult {
        let mut idx: i32 = 0;

        use sys::nvn::types::NVNacquireTextureResult::*;
        match unsafe { ffi::queue_acquire_texture(self.0, window, &mut idx) } {
            Success => QueueAcquireTextureResult::Success { index: idx },
            NativeError => QueueAcquireTextureResult::NativeError,
        }
    }

    #[inline]
    pub fn fence_sync(&mut self, sync: &mut NVNsync, cond: SyncCondition, flush_cpu: bool) {
        unsafe {
            ffi::queue_fence_sync(self.0, sync, cond, flush_cpu as i32);
        }
    }

    #[inline]
    pub fn wait_sync(&mut self, sync: &mut NVNsync) -> bool {
        unsafe { ffi::queue_wait_sync(self.0, sync) }
    }
}
