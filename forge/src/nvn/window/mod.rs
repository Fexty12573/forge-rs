use core::cell::UnsafeCell;
use core::ffi::CStr;
use core::mem::MaybeUninit;

use sys::nvn::types::NVNwindow;

mod builder;

pub use builder::WindowBuilder;

use crate::nvn::owned::impl_object;
use crate::nvn::{NativeWindow, Rectangle, WindowAcquireTextureResult, funcs as ffi, sync::Sync};

#[repr(transparent)]
pub struct Window(UnsafeCell<NVNwindow>);

impl_object!(Window, NVNwindow, ffi::window_finalize);

impl Window {
    #[inline]
    pub fn as_ptr(&self) -> *mut NVNwindow {
        self.0.get()
    }

    #[inline]
    pub fn set_debug_label(&mut self, label: &CStr) {
        unsafe {
            ffi::window_set_debug_label(self.as_ptr(), label.as_ptr());
        }
    }

    #[inline]
    pub fn acquire_texture(&mut self, texture_available: &mut Sync) -> WindowAcquireTextureResult {
        let mut idx: i32 = 0;

        use sys::nvn::types::NVNacquireTextureResult::*;
        match unsafe { ffi::window_acquire_texture(self.as_ptr(), texture_available.as_ptr(), &mut idx) } {
            Success => WindowAcquireTextureResult::Success { index: idx },
            NativeError => WindowAcquireTextureResult::NativeError,
        }
    }

    #[inline]
    pub fn get_native_window(&self) -> NativeWindow {
        unsafe { ffi::window_get_native_window(self.as_ptr()) }
    }

    #[inline]
    pub fn get_present_interval(&self) -> i32 {
        unsafe { ffi::window_get_present_interval(self.as_ptr()) }
    }

    #[inline]
    pub fn set_present_interval(&mut self, interval: i32) {
        unsafe {
            ffi::window_set_present_interval(self.as_ptr(), interval);
        }
    }

    #[inline]
    pub fn set_crop(&mut self, x: i32, y: i32, w: i32, h: i32) {
        unsafe {
            ffi::window_set_crop(self.as_ptr(), x, y, w, h);
        }
    }

    #[inline]
    pub fn get_crop(&self) -> Rectangle {
        let mut rect = MaybeUninit::uninit();
        unsafe {
            ffi::window_get_crop(self.as_ptr(), rect.as_mut_ptr());
            rect.assume_init()
        }
    }
}
