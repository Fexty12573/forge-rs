use sys::imgui::*;

use crate::imgui::ImFont;

#[derive(Clone, Copy)]
pub struct DrawList(*mut sys::imgui::ImDrawList);

impl DrawList {
    pub fn window() -> Self {
        Self(unsafe { igGetWindowDrawList() })
    }

    pub fn foreground() -> Self {
        Self(unsafe { igGetForegroundDrawList_Nil() })
    }

    pub fn background() -> Self {
        Self(unsafe { igGetBackgroundDrawList_Nil() })
    }

    pub fn raw(&self) -> *mut sys::imgui::ImDrawList {
        self.0
    }

    pub fn add_line(&self, p1: ImVec2, p2: ImVec2, col: ImU32, thickness: f32) {
        unsafe { ImDrawList_AddLine(self.0, p1, p2, col, thickness) }
    }

    pub fn add_rect(&self, p_min: ImVec2, p_max: ImVec2, col: ImU32, rounding: f32, thickness: f32, flags: ImDrawFlags) {
        unsafe { ImDrawList_AddRect(self.0, p_min, p_max, col, rounding, thickness, flags) }
    }

    pub fn add_rect_filled(&self, p_min: ImVec2, p_max: ImVec2, col: ImU32, rounding: f32) {
        unsafe { ImDrawList_AddRectFilled(self.0, p_min, p_max, col, rounding, ImDrawFlags::None) }
    }

    pub fn add_rect_filled_ex(&self, p_min: ImVec2, p_max: ImVec2, col: ImU32, rounding: f32, flags: ImDrawFlags) {
        unsafe { ImDrawList_AddRectFilled(self.0, p_min, p_max, col, rounding, flags) }
    }

    pub fn add_rect_filled_multicolor(&self, p_min: ImVec2, p_max: ImVec2, col_upr_left: ImU32, col_upr_right: ImU32, col_bot_right: ImU32, col_bot_left: ImU32) {
        unsafe { ImDrawList_AddRectFilledMultiColor(self.0, p_min, p_max, col_upr_left, col_upr_right, col_bot_right, col_bot_left) }
    }

    pub fn add_triangle(&self, p1: ImVec2, p2: ImVec2, p3: ImVec2, col: ImU32, thickness: f32) {
        unsafe { ImDrawList_AddTriangle(self.0, p1, p2, p3, col, thickness) }
    }

    pub fn add_triangle_filled(&self, p1: ImVec2, p2: ImVec2, p3: ImVec2, col: ImU32) {
        unsafe { ImDrawList_AddTriangleFilled(self.0, p1, p2, p3, col) }
    }

    pub fn add_circle(&self, center: ImVec2, radius: f32, col: ImU32, num_segments: i32, thickness: f32) {
        unsafe { ImDrawList_AddCircle(self.0, center, radius, col, num_segments, thickness) }
    }

    pub fn add_circle_filled(&self, center: ImVec2, radius: f32, col: ImU32, num_segments: i32) {
        unsafe { ImDrawList_AddCircleFilled(self.0, center, radius, col, num_segments) }
    }

    pub fn add_ngon(&self, center: ImVec2, radius: f32, col: ImU32, num_segments: i32, thickness: f32) {
        unsafe { ImDrawList_AddNgon(self.0, center, radius, col, num_segments, thickness) }
    }

    pub fn add_ngon_filled(&self, center: ImVec2, radius: f32, col: ImU32, num_segments: i32) {
        unsafe { ImDrawList_AddNgonFilled(self.0, center, radius, col, num_segments) }
    }

    pub fn add_ellipse(&self, center: ImVec2, radius: ImVec2, col: ImU32, rot: f32, num_segments: i32, thickness: f32) {
        unsafe { ImDrawList_AddEllipse(self.0, center, radius, col, rot, num_segments, thickness) }
    }

    pub fn add_ellipse_filled(&self, center: ImVec2, radius: ImVec2, col: ImU32, rot: f32, num_segments: i32) {
        unsafe { ImDrawList_AddEllipseFilled(self.0, center, radius, col, rot, num_segments) }
    }

    pub fn add_polyline(&self, points: &[ImVec2], col: ImU32, thickness: f32, flags: ImDrawFlags) {
        unsafe { ImDrawList_AddPolyline(self.0, points.as_ptr(), points.len() as _, col, thickness, flags) }
    }

    pub fn add_convex_poly_filled(&self, points: &[ImVec2], col: ImU32) {
        unsafe { ImDrawList_AddConvexPolyFilled(self.0, points.as_ptr(), points.len() as _, col) }
    }

    pub fn add_bezier_cubic(&self, p1: ImVec2, p2: ImVec2, p3: ImVec2, p4: ImVec2, col: ImU32, thickness: f32, num_segments: i32) {
        unsafe { ImDrawList_AddBezierCubic(self.0, p1, p2, p3, p4, col, thickness, num_segments) }
    }

    pub fn add_bezier_quadratic(&self, p1: ImVec2, p2: ImVec2, p3: ImVec2, col: ImU32, thickness: f32, num_segments: i32) {
        unsafe { ImDrawList_AddBezierQuadratic(self.0, p1, p2, p3, col, thickness, num_segments) }
    }

    pub fn add_text(&self, pos: ImVec2, col: ImU32, text: &str) {
        unsafe {
            ImDrawList_AddText_Vec2(
                self.0,
                pos,
                col,
                text.as_ptr(),
                text.as_ptr().add(text.len()),
            )
        }
    }

    pub fn add_text_with_font(&self, font: &ImFont, font_size: f32, pos: ImVec2, col: ImU32, text: &str) {
        unsafe {
            ImDrawList_AddText_FontPtr(
                self.0,
                font.raw(),
                font_size,
                pos,
                col,
                text.as_ptr(),
                text.as_ptr().add(text.len()),
                0.0,
                core::ptr::null(),
            )
        }
    }

    pub fn add_image(&self, tex: ImTextureRef, p_min: ImVec2, p_max: ImVec2, uv_min: ImVec2, uv_max: ImVec2, col: ImU32) {
        unsafe { ImDrawList_AddImage(self.0, tex, p_min, p_max, uv_min, uv_max, col) }
    }

    pub fn push_clip_rect(&self, clip_rect_min: ImVec2, clip_rect_max: ImVec2, intersect_with_current: bool) {
        unsafe { ImDrawList_PushClipRect(self.0, clip_rect_min, clip_rect_max, intersect_with_current) }
    }

    pub fn push_clip_rect_full_screen(&self) {
        unsafe { ImDrawList_PushClipRectFullScreen(self.0) }
    }

    pub fn pop_clip_rect(&self) {
        unsafe { ImDrawList_PopClipRect(self.0) }
    }

    pub fn get_clip_rect_min(&self) -> ImVec2 {
        unsafe { ImDrawList_GetClipRectMin(self.0) }
    }

    pub fn get_clip_rect_max(&self) -> ImVec2 {
        unsafe { ImDrawList_GetClipRectMax(self.0) }
    }

    // Path API

    pub fn path_clear(&self) {
        unsafe { ImDrawList_PathClear(self.0) }
    }

    pub fn path_line_to(&self, pos: ImVec2) {
        unsafe { ImDrawList_PathLineTo(self.0, pos) }
    }

    pub fn path_arc_to(&self, center: ImVec2, radius: f32, a_min: f32, a_max: f32, num_segments: i32) {
        unsafe { ImDrawList_PathArcTo(self.0, center, radius, a_min, a_max, num_segments) }
    }

    pub fn path_arc_to_fast(&self, center: ImVec2, radius: f32, a_min_of_12: i32, a_max_of_12: i32) {
        unsafe { ImDrawList_PathArcToFast(self.0, center, radius, a_min_of_12, a_max_of_12) }
    }

    pub fn path_stroke(&self, col: ImU32, thickness: f32, flags: ImDrawFlags) {
        unsafe { ImDrawList_PathStroke(self.0, col, thickness, flags) }
    }

    pub fn path_fill_convex(&self, col: ImU32) {
        unsafe { ImDrawList_PathFillConvex(self.0, col) }
    }

    pub fn path_fill_concave(&self, col: ImU32) {
        unsafe { ImDrawList_PathFillConcave(self.0, col) }
    }
}
