use sys::imgui::*;

pub fn begin_popup(str_id: &str, flags: ImGuiWindowFlags) -> bool {
    unsafe { igBeginPopup(str_id.as_ptr(), flags) }
}

pub fn begin_popup_modal(name: &str, p_open: Option<&mut bool>, flags: ImGuiWindowFlags) -> bool {
    let p_open_ptr = p_open.map_or(core::ptr::null_mut(), |p| p as *mut bool);
    unsafe { igBeginPopupModal(name.as_ptr(), p_open_ptr, flags) }
}

pub fn end_popup() {
    unsafe { igEndPopup() }
}

pub fn open_popup(str_id: &str, flags: ImGuiPopupFlags) {
    unsafe { igOpenPopup_Str(str_id.as_ptr(), flags) }
}

pub fn open_popup_on_item_click(str_id: Option<&str>, flags: ImGuiPopupFlags) {
    let ptr = str_id.map_or(core::ptr::null(), |s| s.as_ptr());
    unsafe { igOpenPopupOnItemClick(ptr, flags) }
}

pub fn close_current_popup() {
    unsafe { igCloseCurrentPopup() }
}

pub fn begin_popup_context_item(str_id: Option<&str>, flags: ImGuiPopupFlags) -> bool {
    let ptr = str_id.map_or(core::ptr::null(), |s| s.as_ptr());
    unsafe { igBeginPopupContextItem(ptr, flags) }
}

pub fn begin_popup_context_window(str_id: Option<&str>, flags: ImGuiPopupFlags) -> bool {
    let ptr = str_id.map_or(core::ptr::null(), |s| s.as_ptr());
    unsafe { igBeginPopupContextWindow(ptr, flags) }
}

pub fn begin_popup_context_void(str_id: Option<&str>, flags: ImGuiPopupFlags) -> bool {
    let ptr = str_id.map_or(core::ptr::null(), |s| s.as_ptr());
    unsafe { igBeginPopupContextVoid(ptr, flags) }
}

pub fn is_popup_open(str_id: &str, flags: ImGuiPopupFlags) -> bool {
    unsafe { igIsPopupOpen_Str(str_id.as_ptr(), flags) }
}
