use sys::imgui::*;

use crate::imgui::get_io;

pub fn push_font(font: &ImFont, scale: Option<f32>) {
    unsafe { igPushFont(font.raw(), scale.unwrap_or(0.0)) };
}

pub fn pop_font() {
    unsafe { igPopFont() };
}

#[derive(Clone, Copy)]
pub struct ImFont(*mut sys::imgui::ImFont);

impl ImFont {
    pub fn load(path: &str) -> Option<Self> {
        let io = get_io();
        let font = unsafe { ImFontAtlas_AddFontFromFileTTF(io.Fonts, path.as_ptr(), 0.0, core::ptr::null(), core::ptr::null()) };
        if font.is_null() { None } else { Some(Self(font)) }
    }

    pub fn load_with(path: &str, size: f32, cfg: &ImFontConfig, glyph_ranges: Option<&[ImWchar]>) -> Option<Self> {
        let io = get_io();
        let font = unsafe {
            ImFontAtlas_AddFontFromFileTTF(
                io.Fonts,
                path.as_ptr(),
                size,
                cfg as *const ImFontConfig,
                glyph_ranges.map_or(core::ptr::null(), |r| r.as_ptr()),
            )
        };

        if font.is_null() { None } else { Some(Self(font)) }
    }

    pub fn current() -> Self {
        Self(unsafe { igGetFont() })
    }

    pub fn current_size() -> f32 {
        unsafe { igGetFontSize() }
    }

    pub fn raw(&self) -> *mut sys::imgui::ImFont {
        self.0
    }
}
