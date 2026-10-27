use crate::nvn::{NativeWindow, device::Device, funcs as ffi, texture::Texture, window::Window};
use sys::nvn::types::{NVNtexture, NVNwindow, NVNwindowBuilder};

#[repr(transparent)]
pub struct WindowBuilder(pub NVNwindowBuilder);

impl WindowBuilder {
    pub fn new() -> WindowBuilder {
        let mut b = NVNwindowBuilder::default();
        unsafe {
            ffi::window_builder_set_defaults(&mut b);
        }

        WindowBuilder(b)
    }

    pub fn set_device(&mut self, device: &Device) -> &mut Self {
        unsafe {
            ffi::window_builder_set_device(&mut self.0, device.as_ptr());
        }

        self
    }

    pub fn set_native_window(&mut self, native_window: NativeWindow) -> &mut Self {
        unsafe {
            ffi::window_builder_set_native_window(&mut self.0, native_window);
        }

        self
    }

    pub fn set_textures(&mut self, textures: &[&Texture]) -> &mut Self {
        // `&Texture` has the same layout as `*mut NVNtexture`
        unsafe {
            ffi::window_builder_set_textures(
                &mut self.0,
                textures.len() as i32,
                textures.as_ptr().cast::<*mut NVNtexture>(),
            );
        }

        self
    }

    pub fn set_present_interval(&mut self, interval: i32) -> &mut Self {
        unsafe {
            ffi::window_builder_set_present_interval(&mut self.0, interval);
        }

        self
    }

    pub fn get_native_window(&self) -> NativeWindow {
        unsafe { ffi::window_builder_get_native_window(&self.0) }
    }

    pub fn get_present_interval(&self) -> i32 {
        unsafe { ffi::window_builder_get_present_interval(&self.0) }
    }

    pub fn build(&self) -> Window {
        let mut window = NVNwindow::default();
        unsafe {
            ffi::window_initialize(&mut window, &self.0);
        }

        Window(window)
    }
}
