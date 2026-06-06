use core::ffi::CStr;

use sys::imgui::*;

pub fn is_item_hovered(flags: ImGuiHoveredFlags) -> bool {
    unsafe { igIsItemHovered(flags) }
}

pub fn is_item_active() -> bool {
    unsafe { igIsItemActive() }
}

pub fn is_item_focused() -> bool {
    unsafe { igIsItemFocused() }
}

pub fn is_item_clicked(button: ImGuiMouseButton) -> bool {
    unsafe { igIsItemClicked(button) }
}

pub fn is_item_visible() -> bool {
    unsafe { igIsItemVisible() }
}

pub fn is_item_edited() -> bool {
    unsafe { igIsItemEdited() }
}

pub fn is_item_activated() -> bool {
    unsafe { igIsItemActivated() }
}

pub fn is_item_deactivated() -> bool {
    unsafe { igIsItemDeactivated() }
}

pub fn is_item_deactivated_after_edit() -> bool {
    unsafe { igIsItemDeactivatedAfterEdit() }
}

pub fn is_item_toggled_open() -> bool {
    unsafe { igIsItemToggledOpen() }
}

pub fn is_any_item_hovered() -> bool {
    unsafe { igIsAnyItemHovered() }
}

pub fn is_any_item_active() -> bool {
    unsafe { igIsAnyItemActive() }
}

pub fn is_any_item_focused() -> bool {
    unsafe { igIsAnyItemFocused() }
}

pub fn get_item_rect_min() -> ImVec2 {
    unsafe { igGetItemRectMin() }
}

pub fn get_item_rect_max() -> ImVec2 {
    unsafe { igGetItemRectMax() }
}

pub fn get_item_rect_size() -> ImVec2 {
    unsafe { igGetItemRectSize() }
}

pub fn set_item_default_focus() {
    unsafe { igSetItemDefaultFocus() }
}

pub fn set_keyboard_focus_here(offset: i32) {
    unsafe { igSetKeyboardFocusHere(offset) }
}

pub fn set_next_item_allow_overlap() {
    unsafe { igSetNextItemAllowOverlap() }
}

pub fn is_rect_visible(size: ImVec2) -> bool {
    unsafe { igIsRectVisible_Nil(size) }
}

pub fn is_rect_visible_at(rect_min: ImVec2, rect_max: ImVec2) -> bool {
    unsafe { igIsRectVisible_Vec2(rect_min, rect_max) }
}

// Keyboard input

pub fn is_key_down(key: ImGuiKey) -> bool {
    unsafe { igIsKeyDown_Nil(key) }
}

pub fn is_key_pressed(key: ImGuiKey, repeat: bool) -> bool {
    unsafe { igIsKeyPressed_Bool(key, repeat) }
}

pub fn is_key_released(key: ImGuiKey) -> bool {
    unsafe { igIsKeyReleased_Nil(key) }
}

pub fn is_key_chord_pressed(key_chord: ImGuiKeyChord) -> bool {
    unsafe { igIsKeyChordPressed_Nil(key_chord) }
}

pub fn get_key_pressed_amount(key: ImGuiKey, repeat_delay: f32, rate: f32) -> i32 {
    unsafe { igGetKeyPressedAmount(key, repeat_delay, rate) }
}

pub fn get_key_name(key: ImGuiKey) -> &'static str {
    unsafe { CStr::from_ptr(igGetKeyName(key)).to_str().unwrap_or("") }
}

// Mouse input

pub fn is_mouse_down(button: ImGuiMouseButton) -> bool {
    unsafe { igIsMouseDown_Nil(button) }
}

pub fn is_mouse_clicked(button: ImGuiMouseButton, repeat: bool) -> bool {
    unsafe { igIsMouseClicked_Bool(button, repeat) }
}

pub fn is_mouse_released(button: ImGuiMouseButton) -> bool {
    unsafe { igIsMouseReleased_Nil(button) }
}

pub fn is_mouse_double_clicked(button: ImGuiMouseButton) -> bool {
    unsafe { igIsMouseDoubleClicked_Nil(button) }
}

pub fn get_mouse_clicked_count(button: ImGuiMouseButton) -> i32 {
    unsafe { igGetMouseClickedCount(button) }
}

pub fn is_mouse_hovering_rect(r_min: ImVec2, r_max: ImVec2, clip: bool) -> bool {
    unsafe { igIsMouseHoveringRect(r_min, r_max, clip) }
}

pub fn is_mouse_pos_valid() -> bool {
    unsafe { igIsMousePosValid(core::ptr::null()) }
}

pub fn is_mouse_dragging(button: ImGuiMouseButton, lock_threshold: f32) -> bool {
    unsafe { igIsMouseDragging(button, lock_threshold) }
}

pub fn is_any_mouse_down() -> bool {
    unsafe { igIsAnyMouseDown() }
}

pub fn get_mouse_pos() -> ImVec2 {
    unsafe { igGetMousePos() }
}

pub fn get_mouse_drag_delta(button: ImGuiMouseButton, lock_threshold: f32) -> ImVec2 {
    unsafe { igGetMouseDragDelta(button, lock_threshold) }
}

pub fn reset_mouse_drag_delta(button: ImGuiMouseButton) {
    unsafe { igResetMouseDragDelta(button) }
}

pub fn get_mouse_cursor() -> ImGuiMouseCursor {
    unsafe { igGetMouseCursor() }
}

pub fn set_mouse_cursor(cursor_type: ImGuiMouseCursor) {
    unsafe { igSetMouseCursor(cursor_type) }
}

// Capture / misc

pub fn get_time() -> f64 {
    unsafe { igGetTime() }
}

pub fn get_frame_count() -> i32 {
    unsafe { igGetFrameCount() }
}

pub fn get_clipboard_text() -> &'static str {
    let ptr = unsafe { igGetClipboardText() };
    if ptr.is_null() {
        ""
    } else {
        unsafe { CStr::from_ptr(ptr).to_str().unwrap_or("") }
    }
}

pub fn set_clipboard_text(text: &str) {
    unsafe { igSetClipboardText(text.as_ptr()) }
}

pub fn set_next_frame_want_capture_keyboard(capture: bool) {
    unsafe { igSetNextFrameWantCaptureKeyboard(capture) }
}

pub fn set_next_frame_want_capture_mouse(capture: bool) {
    unsafe { igSetNextFrameWantCaptureMouse(capture) }
}
