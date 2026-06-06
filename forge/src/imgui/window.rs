use super::Id;
use sys::imgui::*;

pub fn begin(name: &str, open: Option<&mut bool>, flags: ImGuiWindowFlags) -> bool {
    unsafe { igBegin(name.as_ptr(), open.map_or(core::ptr::null_mut(), |p| p as *mut bool), flags) }
}

pub fn end() {
    unsafe { igEnd() };
}

pub fn begin_child(id: Id, size: ImVec2, child_flags: ImGuiChildFlags, window_flags: ImGuiWindowFlags) -> bool {
    match id {
        Id::Int(id) => unsafe { igBeginChild_ID(id, size, child_flags, window_flags) },
        Id::Str(str_id) => unsafe { igBeginChild_Str(str_id.as_ptr(), size, child_flags, window_flags) },
    }
}

pub fn end_child() {
    unsafe { igEndChild() };
}

pub fn is_window_appearing() -> bool {
    unsafe { igIsWindowAppearing() }
}

pub fn is_window_collapsed() -> bool {
    unsafe { igIsWindowCollapsed() }
}

pub fn is_window_focused(flags: ImGuiFocusedFlags) -> bool {
    unsafe { igIsWindowFocused(flags) }
}

pub fn is_window_hovered(flags: ImGuiHoveredFlags) -> bool {
    unsafe { igIsWindowHovered(flags) }
}

pub fn set_next_window_pos(pos: ImVec2, cond: ImGuiCond, pivot: ImVec2) {
    unsafe { igSetNextWindowPos(pos, cond, pivot) };
}

pub fn set_next_window_size(size: ImVec2, cond: ImGuiCond) {
    unsafe { igSetNextWindowSize(size, cond) };
}

pub fn set_next_window_collapsed(collapsed: bool, cond: ImGuiCond) {
    unsafe { igSetNextWindowCollapsed(collapsed, cond) };
}

pub fn set_next_window_focus() {
    unsafe { igSetNextWindowFocus() };
}

pub fn set_next_window_scroll(scroll: ImVec2) {
    unsafe { igSetNextWindowScroll(scroll) };
}

pub fn set_next_window_bg_alpha(alpha: f32) {
    unsafe { igSetNextWindowBgAlpha(alpha) };
}

pub fn set_next_window_content_size(size: ImVec2) {
    unsafe { igSetNextWindowContentSize(size) };
}

pub fn set_next_window_size_constraints(size_min: ImVec2, size_max: ImVec2) {
    unsafe { igSetNextWindowSizeConstraints(size_min, size_max, None, core::ptr::null_mut()) };
}

pub fn get_window_pos() -> ImVec2 {
    unsafe { igGetWindowPos() }
}

pub fn get_window_size() -> ImVec2 {
    unsafe { igGetWindowSize() }
}

pub fn get_window_width() -> f32 {
    unsafe { igGetWindowWidth() }
}

pub fn get_window_height() -> f32 {
    unsafe { igGetWindowHeight() }
}
