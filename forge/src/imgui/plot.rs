use core::ffi::CStr;

use sys::imgui::*;

pub fn plot_lines(
    label: &CStr,
    values: &[f32],
    values_offset: i32,
    overlay: Option<&CStr>,
    scale_min: f32,
    scale_max: f32,
    graph_size: ImVec2,
) {
    let overlay_ptr = overlay.map_or(core::ptr::null(), |s| s.as_ptr());
    unsafe {
        igPlotLines_FloatPtr(
            label.as_ptr(),
            values.as_ptr(),
            values.len() as _,
            values_offset,
            overlay_ptr,
            scale_min,
            scale_max,
            graph_size,
            core::mem::size_of::<f32>() as _,
        )
    }
}

pub fn plot_histogram(
    label: &CStr,
    values: &[f32],
    values_offset: i32,
    overlay: Option<&CStr>,
    scale_min: f32,
    scale_max: f32,
    graph_size: ImVec2,
) {
    let overlay_ptr = overlay.map_or(core::ptr::null(), |s| s.as_ptr());
    unsafe {
        igPlotHistogram_FloatPtr(
            label.as_ptr(),
            values.as_ptr(),
            values.len() as _,
            values_offset,
            overlay_ptr,
            scale_min,
            scale_max,
            graph_size,
            core::mem::size_of::<f32>() as _,
        )
    }
}
