use core::ffi::CStr;
use core::mem::MaybeUninit;

use sys::nvn::types::NVNwindow;

mod builder;

pub use builder::WindowBuilder;

use crate::nvn::{NativeWindow, Rectangle, WindowAcquireTextureResult, funcs as ffi, sync::Sync};

#[repr(transparent)]
pub struct Window(pub NVNwindow);

impl Window {
    #[inline]
    pub fn set_debug_label(&mut self, label: &CStr) {
        unsafe {
            ffi::window_set_debug_label(&mut self.0, label.as_ptr());
        }
    }

    #[inline]
    pub fn acquire_texture(&mut self, texture_available: &mut Sync) -> WindowAcquireTextureResult {
        let mut idx: i32 = 0;

        use sys::nvn::types::NVNacquireTextureResult::*;
        match unsafe { ffi::window_acquire_texture(&mut self.0, &mut texture_available.0, &mut idx) } {
            Success => WindowAcquireTextureResult::Success { index: idx },
            NativeError => WindowAcquireTextureResult::NativeError,
        }
    }

    #[inline]
    pub fn get_native_window(&self) -> NativeWindow {
        unsafe { ffi::window_get_native_window(&self.0) }
    }

    #[inline]
    pub fn get_present_interval(&self) -> i32 {
        unsafe { ffi::window_get_present_interval(&self.0) }
    }

    #[inline]
    pub fn set_present_interval(&mut self, interval: i32) {
        unsafe {
            ffi::window_set_present_interval(&mut self.0, interval);
        }
    }

    #[inline]
    pub fn set_crop(&mut self, x: i32, y: i32, w: i32, h: i32) {
        unsafe {
            ffi::window_set_crop(&mut self.0, x, y, w, h);
        }
    }

    #[inline]
    pub fn get_crop(&self) -> Rectangle {
        let mut rect = MaybeUninit::uninit();
        unsafe {
            ffi::window_get_crop(&self.0, rect.as_mut_ptr());
            rect.assume_init()
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            ffi::window_finalize(&mut self.0);
        }
    }
}
