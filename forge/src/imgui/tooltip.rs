use sys::imgui::*;

pub fn begin_tooltip() -> bool {
    unsafe { igBeginTooltip() }
}

pub fn end_tooltip() {
    unsafe { igEndTooltip() }
}

pub fn set_tooltip(s: &str) {
    unsafe {
        if igBeginTooltip() {
            igTextUnformatted(s.as_ptr(), s.as_ptr().add(s.len()));
            igEndTooltip();
        }
    }
}

pub fn begin_item_tooltip() -> bool {
    unsafe { igBeginItemTooltip() }
}

pub fn set_item_tooltip(s: &str) {
    unsafe { igSetItemTooltip(b"%s\0".as_ptr(), s.as_ptr()) }
}
