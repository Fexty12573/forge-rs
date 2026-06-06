use sys::imgui::*;

/// Packs RGBA bytes (0-255 each) into a Dear ImGui `ImU32` color.
pub fn color(r: u8, g: u8, b: u8, a: u8) -> ImU32 {
    (r as u32) | ((g as u32) << 8) | ((b as u32) << 16) | ((a as u32) << 24)
}

pub fn color_f32(r: f32, g: f32, b: f32, a: f32) -> ImU32 {
    unsafe { igColorConvertFloat4ToU32(ImVec4::new(r, g, b, a)) }
}

pub fn get_color_u32(idx: ImGuiCol, alpha_mul: f32) -> ImU32 {
    unsafe { igGetColorU32_Col(idx, alpha_mul) }
}

pub fn get_color_u32_vec4(col: ImVec4) -> ImU32 {
    unsafe { igGetColorU32_Vec4(col) }
}

pub fn get_color_u32_u32(col: ImU32, alpha_mul: f32) -> ImU32 {
    unsafe { igGetColorU32_U32(col, alpha_mul) }
}

pub fn color_edit3(label: &str, col: &mut [f32; 3], flags: ImGuiColorEditFlags) -> bool {
    unsafe { igColorEdit3(label.as_ptr(), col.as_mut_ptr(), flags) }
}

pub fn color_edit4(label: &str, col: &mut [f32; 4], flags: ImGuiColorEditFlags) -> bool {
    unsafe { igColorEdit4(label.as_ptr(), col.as_mut_ptr(), flags) }
}

pub fn color_picker3(label: &str, col: &mut [f32; 3], flags: ImGuiColorEditFlags) -> bool {
    unsafe { igColorPicker3(label.as_ptr(), col.as_mut_ptr(), flags) }
}

pub fn color_picker4(label: &str, col: &mut [f32; 4], flags: ImGuiColorEditFlags) -> bool {
    unsafe { igColorPicker4(label.as_ptr(), col.as_mut_ptr(), flags, core::ptr::null()) }
}

pub fn color_convert_rgb_to_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let (mut h, mut s, mut v) = (0.0f32, 0.0f32, 0.0f32);
    unsafe { igColorConvertRGBtoHSV(r, g, b, &mut h, &mut s, &mut v) };
    (h, s, v)
}

pub fn color_convert_hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let (mut r, mut g, mut b) = (0.0f32, 0.0f32, 0.0f32);
    unsafe { igColorConvertHSVtoRGB(h, s, v, &mut r, &mut g, &mut b) };
    (r, g, b)
}

pub fn set_color_edit_options(flags: ImGuiColorEditFlags) {
    unsafe { igSetColorEditOptions(flags) }
}
