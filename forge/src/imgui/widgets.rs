use core::ffi::CStr;

use sys::imgui::*;

pub fn text(s: &str) {
    unsafe { igTextUnformatted(s.as_ptr(), s.as_ptr().add(s.len())) }
}

pub fn text_colored(col: ImVec4, s: &CStr) {
    unsafe { igTextColored(col, b"%s\0".as_ptr(), s.as_ptr()) }
}

pub fn text_disabled(s: &CStr) {
    unsafe { igTextDisabled(b"%s\0".as_ptr(), s.as_ptr()) }
}

pub fn text_wrapped(s: &CStr) {
    unsafe { igTextWrapped(b"%s\0".as_ptr(), s.as_ptr()) }
}

pub fn label_text(label: &CStr, value: &CStr) {
    unsafe { igLabelText(label.as_ptr(), b"%s\0".as_ptr(), value.as_ptr()) }
}

pub fn separator_text(label: &CStr) {
    unsafe { igSeparatorText(label.as_ptr()) }
}

pub fn text_link(label: &CStr) -> bool {
    unsafe { igTextLink(label.as_ptr()) }
}

pub fn button(label: &CStr, size: ImVec2) -> bool {
    unsafe { igButton(label.as_ptr(), size) }
}

pub fn small_button(label: &CStr) -> bool {
    unsafe { igSmallButton(label.as_ptr()) }
}

pub fn invisible_button(str_id: &CStr, size: ImVec2, flags: ImGuiButtonFlags) -> bool {
    unsafe { igInvisibleButton(str_id.as_ptr(), size, flags) }
}

pub fn arrow_button(str_id: &CStr, dir: ImGuiDir) -> bool {
    unsafe { igArrowButton(str_id.as_ptr(), dir) }
}

pub fn checkbox(label: &CStr, v: &mut bool) -> bool {
    unsafe { igCheckbox(label.as_ptr(), v as *mut bool) }
}

pub fn checkbox_flags(label: &CStr, flags: &mut u32, flags_value: u32) -> bool {
    unsafe { igCheckboxFlags_UintPtr(label.as_ptr(), flags as *mut u32, flags_value) }
}

pub fn radio_button(label: &CStr, active: bool) -> bool {
    unsafe { igRadioButton_Bool(label.as_ptr(), active) }
}

pub fn radio_button_int(label: &CStr, v: &mut i32, v_button: i32) -> bool {
    unsafe { igRadioButton_IntPtr(label.as_ptr(), v as *mut _, v_button) }
}

pub fn selectable(label: &CStr, selected: bool, flags: ImGuiSelectableFlags, size: ImVec2) -> bool {
    unsafe { igSelectable_Bool(label.as_ptr(), selected, flags, size) }
}

pub fn selectable_toggle(label: &CStr, p_selected: &mut bool, flags: ImGuiSelectableFlags, size: ImVec2) -> bool {
    unsafe { igSelectable_BoolPtr(label.as_ptr(), p_selected as *mut bool, flags, size) }
}

pub fn begin_combo(label: &CStr, preview_value: &CStr, flags: ImGuiComboFlags) -> bool {
    unsafe { igBeginCombo(label.as_ptr(), preview_value.as_ptr(), flags) }
}

pub fn end_combo() {
    unsafe { igEndCombo() }
}

pub fn begin_list_box(label: &CStr, size: ImVec2) -> bool {
    unsafe { igBeginListBox(label.as_ptr(), size) }
}

pub fn end_list_box() {
    unsafe { igEndListBox() }
}

pub fn progress_bar(fraction: f32, size: ImVec2, overlay: Option<&CStr>) {
    let overlay_ptr = overlay.map_or(core::ptr::null(), |s| s.as_ptr());
    unsafe { igProgressBar(fraction, size, overlay_ptr) }
}

pub fn bullet() {
    unsafe { igBullet() }
}

pub fn bullet_text(s: &CStr) {
    unsafe { igBulletText(b"%s\0".as_ptr(), s.as_ptr()) }
}

pub fn image(tex: ImTextureRef, size: ImVec2, uv0: ImVec2, uv1: ImVec2) {
    unsafe { igImage(tex, size, uv0, uv1) }
}

pub fn image_with_bg(tex: ImTextureRef, size: ImVec2, uv0: ImVec2, uv1: ImVec2, bg_col: ImVec4, tint_col: ImVec4) {
    unsafe { igImageWithBg(tex, size, uv0, uv1, bg_col, tint_col) }
}

pub fn image_button(str_id: &CStr, tex: ImTextureRef, size: ImVec2, uv0: ImVec2, uv1: ImVec2, bg_col: ImVec4, tint_col: ImVec4) -> bool {
    unsafe { igImageButton(str_id.as_ptr(), tex, size, uv0, uv1, bg_col, tint_col) }
}

pub fn color_button(desc_id: &CStr, col: ImVec4, flags: ImGuiColorEditFlags, size: ImVec2) -> bool {
    unsafe { igColorButton(desc_id.as_ptr(), col, flags, size) }
}
