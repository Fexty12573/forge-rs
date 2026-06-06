use super::Id;
use sys::imgui::*;

pub fn push_style_color(idx: ImGuiCol, col: ImVec4) {
    unsafe { igPushStyleColor_Vec4(idx, col) };
}

pub fn pop_style_color(count: i32) {
    unsafe { igPopStyleColor(count) };
}

pub fn push_style_var_float(idx: ImGuiStyleVar, val: f32) {
    unsafe { igPushStyleVar_Float(idx, val) };
}

pub fn push_style_var_vec2(idx: ImGuiStyleVar, val: ImVec2) {
    unsafe { igPushStyleVar_Vec2(idx, val) };
}

pub fn pop_style_var(count: i32) {
    unsafe { igPopStyleVar(count) };
}

pub fn push_item_flag(flags: ImGuiItemFlags, enabled: bool) {
    unsafe { igPushItemFlag(flags, enabled) };
}

pub fn pop_item_flag() {
    unsafe { igPopItemFlag() };
}

pub fn push_item_width(width: f32) {
    unsafe { igPushItemWidth(width) };
}

pub fn pop_item_width() {
    unsafe { igPopItemWidth() };
}

pub fn push_id(id: Id) {
    match id {
        Id::Int(id) => unsafe { igPushID_Int(id as i32) },
        Id::Str(str_id) => unsafe { igPushID_Str(str_id.as_ptr()) },
    }
}

pub fn pop_id() {
    unsafe { igPopID() };
}

pub fn get_id(id: Id) -> u32 {
    match id {
        Id::Int(id) => unsafe { igGetID_Int(id as i32) },
        Id::Str(str_id) => unsafe { igGetID_Str(str_id.as_ptr()) },
    }
}
