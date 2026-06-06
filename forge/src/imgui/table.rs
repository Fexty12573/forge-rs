use core::ffi::CStr;

use sys::imgui::*;

pub fn begin_table(str_id: &str, columns: i32, flags: ImGuiTableFlags, outer_size: ImVec2, inner_width: f32) -> bool {
    unsafe { igBeginTable(str_id.as_ptr(), columns, flags, outer_size, inner_width) }
}

pub fn end_table() {
    unsafe { igEndTable() }
}

pub fn table_next_row(flags: ImGuiTableRowFlags, min_row_height: f32) {
    unsafe { igTableNextRow(flags, min_row_height) }
}

pub fn table_next_column() -> bool {
    unsafe { igTableNextColumn() }
}

pub fn table_set_column_index(column: i32) -> bool {
    unsafe { igTableSetColumnIndex(column) }
}

pub fn table_setup_column(label: &str, flags: ImGuiTableColumnFlags, init_width_or_weight: f32, user_id: ImGuiID) {
    unsafe { igTableSetupColumn(label.as_ptr(), flags, init_width_or_weight, user_id) }
}

pub fn table_setup_scroll_freeze(cols: i32, rows: i32) {
    unsafe { igTableSetupScrollFreeze(cols, rows) }
}

pub fn table_headers_row() {
    unsafe { igTableHeadersRow() }
}

pub fn table_angled_headers_row() {
    unsafe { igTableAngledHeadersRow() }
}

pub fn table_header(label: &str) {
    unsafe { igTableHeader(label.as_ptr()) }
}

pub fn table_get_sort_specs() -> Option<&'static mut ImGuiTableSortSpecs> {
    let ptr = unsafe { igTableGetSortSpecs() };
    if ptr.is_null() { None } else { Some(unsafe { &mut *ptr }) }
}

pub fn table_get_column_count() -> i32 {
    unsafe { igTableGetColumnCount() }
}

pub fn table_get_column_index() -> i32 {
    unsafe { igTableGetColumnIndex() }
}

pub fn table_get_row_index() -> i32 {
    unsafe { igTableGetRowIndex() }
}

pub fn table_get_column_name(column: i32) -> &'static str {
    let ptr = unsafe { igTableGetColumnName_Int(column) };
    if ptr.is_null() {
        ""
    } else {
        unsafe { CStr::from_ptr(ptr).to_str().unwrap_or("") }
    }
}

pub fn table_get_column_flags(column: i32) -> ImGuiTableColumnFlags {
    unsafe { igTableGetColumnFlags(column) }
}

pub fn table_set_column_enabled(column: i32, enabled: bool) {
    unsafe { igTableSetColumnEnabled(column, enabled) }
}

pub fn table_set_bg_color(target: ImGuiTableBgTarget, color: ImU32, column: i32) {
    unsafe { igTableSetBgColor(target, color, column) }
}

pub fn table_get_hovered_column() -> i32 {
    unsafe { igTableGetHoveredColumn() }
}

pub fn table_get_hovered_row() -> i32 {
    unsafe { igTableGetHoveredRow() }
}
