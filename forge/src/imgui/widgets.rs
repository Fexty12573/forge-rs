use sys::imgui::*;

pub fn text(s: &str) {
    unsafe { igTextUnformatted(s.as_ptr(), s.as_ptr().add(s.len())) }
}

pub fn text_colored(col: ImVec4, s: &str) {
    unsafe { igTextColored(col, b"%s\0".as_ptr(), s.as_ptr()) }
}

pub fn text_disabled(s: &str) {
    unsafe { igTextDisabled(b"%s\0".as_ptr(), s.as_ptr()) }
}

pub fn text_wrapped(s: &str) {
    unsafe { igTextWrapped(b"%s\0".as_ptr(), s.as_ptr()) }
}

pub fn label_text(label: &str, value: &str) {
    unsafe { igLabelText(label.as_ptr(), b"%s\0".as_ptr(), value.as_ptr()) }
}

pub fn separator_text(label: &str) {
    unsafe { igSeparatorText(label.as_ptr()) }
}

pub fn text_link(label: &str) -> bool {
    unsafe { igTextLink(label.as_ptr()) }
}

pub fn button(label: &str, size: ImVec2) -> bool {
    unsafe { igButton(label.as_ptr(), size) }
}

pub fn small_button(label: &str) -> bool {
    unsafe { igSmallButton(label.as_ptr()) }
}

pub fn invisible_button(str_id: &str, size: ImVec2, flags: ImGuiButtonFlags) -> bool {
    unsafe { igInvisibleButton(str_id.as_ptr(), size, flags) }
}

pub fn arrow_button(str_id: &str, dir: ImGuiDir) -> bool {
    unsafe { igArrowButton(str_id.as_ptr(), dir) }
}

pub fn checkbox(label: &str, v: &mut bool) -> bool {
    unsafe { igCheckbox(label.as_ptr(), v as *mut bool) }
}

pub fn checkbox_flags(label: &str, flags: &mut u32, flags_value: u32) -> bool {
    unsafe { igCheckboxFlags_UintPtr(label.as_ptr(), flags as *mut u32, flags_value) }
}

pub fn radio_button(label: &str, active: bool) -> bool {
    unsafe { igRadioButton_Bool(label.as_ptr(), active) }
}

pub fn radio_button_int(label: &str, v: &mut i32, v_button: i32) -> bool {
    unsafe { igRadioButton_IntPtr(label.as_ptr(), v as *mut _, v_button) }
}

pub fn selectable(label: &str, selected: bool, flags: ImGuiSelectableFlags, size: ImVec2) -> bool {
    unsafe { igSelectable_Bool(label.as_ptr(), selected, flags, size) }
}

pub fn selectable_toggle(label: &str, p_selected: &mut bool, flags: ImGuiSelectableFlags, size: ImVec2) -> bool {
    unsafe { igSelectable_BoolPtr(label.as_ptr(), p_selected as *mut bool, flags, size) }
}

pub fn begin_combo(label: &str, preview_value: &str, flags: ImGuiComboFlags) -> bool {
    unsafe { igBeginCombo(label.as_ptr(), preview_value.as_ptr(), flags) }
}

pub fn end_combo() {
    unsafe { igEndCombo() }
}

pub fn begin_list_box(label: &str, size: ImVec2) -> bool {
    unsafe { igBeginListBox(label.as_ptr(), size) }
}

pub fn end_list_box() {
    unsafe { igEndListBox() }
}

pub fn progress_bar(fraction: f32, size: ImVec2, overlay: Option<&str>) {
    let overlay_ptr = overlay.map_or(core::ptr::null(), |s| s.as_ptr());
    unsafe { igProgressBar(fraction, size, overlay_ptr) }
}

pub fn bullet() {
    unsafe { igBullet() }
}

pub fn bullet_text(s: &str) {
    unsafe { igBulletText(b"%s\0".as_ptr(), s.as_ptr()) }
}

pub fn image(tex: ImTextureRef, size: ImVec2, uv0: ImVec2, uv1: ImVec2) {
    unsafe { igImage(tex, size, uv0, uv1) }
}

pub fn image_with_bg(tex: ImTextureRef, size: ImVec2, uv0: ImVec2, uv1: ImVec2, bg_col: ImVec4, tint_col: ImVec4) {
    unsafe { igImageWithBg(tex, size, uv0, uv1, bg_col, tint_col) }
}

pub fn image_button(str_id: &str, tex: ImTextureRef, size: ImVec2, uv0: ImVec2, uv1: ImVec2, bg_col: ImVec4, tint_col: ImVec4) -> bool {
    unsafe { igImageButton(str_id.as_ptr(), tex, size, uv0, uv1, bg_col, tint_col) }
}

pub fn color_button(desc_id: &str, col: ImVec4, flags: ImGuiColorEditFlags, size: ImVec2) -> bool {
    unsafe { igColorButton(desc_id.as_ptr(), col, flags, size) }
}
