use sys::imgui::*;

pub fn same_line(offset_from_start_x: f32, spacing: f32) {
    unsafe { igSameLine(offset_from_start_x, spacing) }
}

pub fn new_line() {
    unsafe { igNewLine() }
}

pub fn spacing() {
    unsafe { igSpacing() }
}

pub fn dummy(size: ImVec2) {
    unsafe { igDummy(size) }
}

pub fn separator() {
    unsafe { igSeparator() }
}

pub fn indent(indent_w: f32) {
    unsafe { igIndent(indent_w) }
}

pub fn unindent(indent_w: f32) {
    unsafe { igUnindent(indent_w) }
}

pub fn begin_group() {
    unsafe { igBeginGroup() }
}

pub fn end_group() {
    unsafe { igEndGroup() }
}

pub fn align_text_to_frame_padding() {
    unsafe { igAlignTextToFramePadding() }
}

pub fn push_text_wrap_pos(wrap_local_pos_x: f32) {
    unsafe { igPushTextWrapPos(wrap_local_pos_x) }
}

pub fn pop_text_wrap_pos() {
    unsafe { igPopTextWrapPos() }
}

pub fn set_next_item_width(item_width: f32) {
    unsafe { igSetNextItemWidth(item_width) }
}

pub fn calc_text_size(text: &str, hide_text_after_double_hash: bool, wrap_width: f32) -> ImVec2 {
    unsafe {
        igCalcTextSize(
            text.as_ptr(),
            text.as_ptr().add(text.len()),
            hide_text_after_double_hash,
            wrap_width,
        )
    }
}

pub fn get_content_region_avail() -> ImVec2 {
    unsafe { igGetContentRegionAvail() }
}

pub fn get_cursor_pos() -> ImVec2 {
    unsafe { igGetCursorPos() }
}

pub fn get_cursor_pos_x() -> f32 {
    unsafe { igGetCursorPosX() }
}

pub fn get_cursor_pos_y() -> f32 {
    unsafe { igGetCursorPosY() }
}

pub fn set_cursor_pos(pos: ImVec2) {
    unsafe { igSetCursorPos(pos) }
}

pub fn set_cursor_pos_x(x: f32) {
    unsafe { igSetCursorPosX(x) }
}

pub fn set_cursor_pos_y(y: f32) {
    unsafe { igSetCursorPosY(y) }
}

pub fn get_cursor_screen_pos() -> ImVec2 {
    unsafe { igGetCursorScreenPos() }
}

pub fn set_cursor_screen_pos(pos: ImVec2) {
    unsafe { igSetCursorScreenPos(pos) }
}

pub fn get_cursor_start_pos() -> ImVec2 {
    unsafe { igGetCursorStartPos() }
}

pub fn get_text_line_height() -> f32 {
    unsafe { igGetTextLineHeight() }
}

pub fn get_text_line_height_with_spacing() -> f32 {
    unsafe { igGetTextLineHeightWithSpacing() }
}

pub fn get_frame_height() -> f32 {
    unsafe { igGetFrameHeight() }
}

pub fn get_frame_height_with_spacing() -> f32 {
    unsafe { igGetFrameHeightWithSpacing() }
}

pub fn get_scroll_x() -> f32 {
    unsafe { igGetScrollX() }
}

pub fn get_scroll_y() -> f32 {
    unsafe { igGetScrollY() }
}

pub fn set_scroll_x(scroll_x: f32) {
    unsafe { igSetScrollX_Float(scroll_x) }
}

pub fn set_scroll_y(scroll_y: f32) {
    unsafe { igSetScrollY_Float(scroll_y) }
}

pub fn get_scroll_max_x() -> f32 {
    unsafe { igGetScrollMaxX() }
}

pub fn get_scroll_max_y() -> f32 {
    unsafe { igGetScrollMaxY() }
}

pub fn set_scroll_here_x(center_x_ratio: f32) {
    unsafe { igSetScrollHereX(center_x_ratio) }
}

pub fn set_scroll_here_y(center_y_ratio: f32) {
    unsafe { igSetScrollHereY(center_y_ratio) }
}

pub fn set_scroll_from_pos_x(local_x: f32, center_x_ratio: f32) {
    unsafe { igSetScrollFromPosX_Float(local_x, center_x_ratio) }
}

pub fn set_scroll_from_pos_y(local_y: f32, center_y_ratio: f32) {
    unsafe { igSetScrollFromPosY_Float(local_y, center_y_ratio) }
}

pub fn begin_disabled(disabled: bool) {
    unsafe { igBeginDisabled(disabled) }
}

pub fn end_disabled() {
    unsafe { igEndDisabled() }
}

pub fn push_clip_rect(clip_rect_min: ImVec2, clip_rect_max: ImVec2, intersect_with_current: bool) {
    unsafe { igPushClipRect(clip_rect_min, clip_rect_max, intersect_with_current) }
}

pub fn pop_clip_rect() {
    unsafe { igPopClipRect() }
}
