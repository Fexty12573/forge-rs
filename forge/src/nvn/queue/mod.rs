mod builder;

use core::{cell::UnsafeCell, ffi::CStr};

pub use builder::QueueBuilder;
use sys::nvn::types::{NVNqueue, NVNsync, NVNwindow};

use crate::nvn::{CommandHandle, QueueAcquireTextureResult, QueueErrorInfo, QueueGetErrorResult, SyncCondition, funcs as ffi};

// TODO: replace NVN* types with proper counterparts

#[repr(transparent)]
pub struct Queue(UnsafeCell<NVNqueue>);

impl Queue {
    /// # Safety
    /// `ptr` must be non-null and point to an initialized `NVNdevice` that
    /// stays valid for `'a`.
    pub fn from_ptr_mut<'a>(ptr: *mut NVNqueue) -> &'a mut Self {
        unsafe { &mut *ptr.cast() }
    }

    /// # Safety
    /// `ptr` must be non-null and point to an initialized `NVNdevice` that
    /// stays valid for `'a`.
    pub fn from_ptr<'a>(ptr: *const NVNqueue) -> &'a Self {
        unsafe { &*ptr.cast() }
    }

    #[inline]
    pub fn as_ptr(&self) -> *mut NVNqueue {
        self.0.get()
    }

    #[inline]
    pub fn initialize(&mut self, builder: &QueueBuilder) -> bool {
        unsafe { ffi::queue_initialize(self.as_ptr(), &builder.0) }
    }

    #[inline]
    pub fn finalize(&mut self) {
        unsafe {
            ffi::queue_finalize(self.as_ptr());
        }
    }

    #[inline]
    pub fn set_debug_label(&mut self, label: &CStr) {
        unsafe {
            ffi::queue_set_debug_label(self.as_ptr(), label.as_ptr());
        }
    }

    #[inline]
    pub fn total_command_mem_used(&self) -> usize {
        unsafe { ffi::queue_get_total_command_memory_used(self.as_ptr()) }
    }

    #[inline]
    pub fn total_control_mem_used(&self) -> usize {
        unsafe { ffi::queue_get_total_control_memory_used(self.as_ptr()) }
    }

    #[inline]
    pub fn total_compute_mem_used(&self) -> usize {
        unsafe { ffi::queue_get_total_compute_memory_used(self.as_ptr()) }
    }

    #[inline]
    pub fn get_error(&self, info: *mut QueueErrorInfo) -> QueueGetErrorResult {
        unsafe { ffi::queue_get_error(self.as_ptr(), info) }
    }

    #[inline]
    pub fn reset_mem_usage_counts(&mut self) {
        unsafe {
            ffi::queue_reset_memory_usage_counts(self.as_ptr());
        }
    }

    #[inline]
    pub fn submit(&mut self, cmds: &[CommandHandle]) {
        unsafe {
            ffi::queue_submit_commands(self.as_ptr(), cmds.len() as i32, cmds.as_ptr());
        }
    }

    #[inline]
    pub fn flush(&mut self) {
        unsafe {
            ffi::queue_flush(self.as_ptr());
        }
    }

    #[inline]
    pub fn finish(&mut self) {
        unsafe {
            ffi::queue_finish(self.as_ptr());
        }
    }

    #[inline]
    pub fn present_texture(&mut self, window: &mut NVNwindow, tex_id: i32) {
        unsafe {
            ffi::queue_present_texture(self.as_ptr(), &mut *window, tex_id);
        }
    }

    #[inline]
    pub fn acquire_texture(&mut self, window: &mut NVNwindow) -> QueueAcquireTextureResult {
        let mut idx: i32 = 0;

        use sys::nvn::types::NVNacquireTextureResult::*;
        match unsafe { ffi::queue_acquire_texture(self.as_ptr(), window, &mut idx) } {
            Success => QueueAcquireTextureResult::Success { index: idx },
            NativeError => QueueAcquireTextureResult::NativeError,
        }
    }

    #[inline]
    pub fn fence_sync(&mut self, sync: &mut NVNsync, cond: SyncCondition, flush_cpu: bool) {
        unsafe {
            ffi::queue_fence_sync(self.as_ptr(), sync, cond, flush_cpu as i32);
        }
    }

    #[inline]
    pub fn wait_sync(&mut self, sync: &mut NVNsync) -> bool {
        unsafe { ffi::queue_wait_sync(self.as_ptr(), sync) }
    }
}
