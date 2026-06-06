use sys::imgui::*;

pub fn begin_menu_bar() -> bool {
    unsafe { igBeginMenuBar() }
}

pub fn end_menu_bar() {
    unsafe { igEndMenuBar() }
}

pub fn begin_main_menu_bar() -> bool {
    unsafe { igBeginMainMenuBar() }
}

pub fn end_main_menu_bar() {
    unsafe { igEndMainMenuBar() }
}

pub fn begin_menu(label: &str, enabled: bool) -> bool {
    unsafe { igBeginMenu(label.as_ptr(), enabled) }
}

pub fn end_menu() {
    unsafe { igEndMenu() }
}

pub fn menu_item(label: &str, shortcut: Option<&str>, selected: bool, enabled: bool) -> bool {
    let shortcut_ptr = shortcut.map_or(core::ptr::null(), |s| s.as_ptr());
    unsafe { igMenuItem_Bool(label.as_ptr(), shortcut_ptr, selected, enabled) }
}

pub fn menu_item_toggle(label: &str, shortcut: Option<&str>, p_selected: &mut bool, enabled: bool) -> bool {
    let shortcut_ptr = shortcut.map_or(core::ptr::null(), |s| s.as_ptr());
    unsafe { igMenuItem_BoolPtr(label.as_ptr(), shortcut_ptr, p_selected as *mut bool, enabled) }
}
