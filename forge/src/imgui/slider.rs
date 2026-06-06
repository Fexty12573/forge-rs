use sys::imgui::*;

pub fn slider_float(label: &str, v: &mut f32, min: f32, max: f32, flags: ImGuiSliderFlags) -> bool {
    unsafe { igSliderFloat(label.as_ptr(), v as *mut f32, min, max, core::ptr::null(), flags) }
}

pub fn slider_float2(label: &str, v: &mut [f32; 2], min: f32, max: f32, flags: ImGuiSliderFlags) -> bool {
    unsafe { igSliderFloat2(label.as_ptr(), v.as_mut_ptr(), min, max, core::ptr::null(), flags) }
}

pub fn slider_float3(label: &str, v: &mut [f32; 3], min: f32, max: f32, flags: ImGuiSliderFlags) -> bool {
    unsafe { igSliderFloat3(label.as_ptr(), v.as_mut_ptr(), min, max, core::ptr::null(), flags) }
}

pub fn slider_float4(label: &str, v: &mut [f32; 4], min: f32, max: f32, flags: ImGuiSliderFlags) -> bool {
    unsafe { igSliderFloat4(label.as_ptr(), v.as_mut_ptr(), min, max, core::ptr::null(), flags) }
}

pub fn slider_int(label: &str, v: &mut i32, min: i32, max: i32, flags: ImGuiSliderFlags) -> bool {
    unsafe { igSliderInt(label.as_ptr(), v as *mut _, min, max, core::ptr::null(), flags) }
}

pub fn slider_int2(label: &str, v: &mut [i32; 2], min: i32, max: i32, flags: ImGuiSliderFlags) -> bool {
    unsafe { igSliderInt2(label.as_ptr(), v.as_mut_ptr(), min, max, core::ptr::null(), flags) }
}

pub fn slider_int3(label: &str, v: &mut [i32; 3], min: i32, max: i32, flags: ImGuiSliderFlags) -> bool {
    unsafe { igSliderInt3(label.as_ptr(), v.as_mut_ptr(), min, max, core::ptr::null(), flags) }
}

pub fn slider_int4(label: &str, v: &mut [i32; 4], min: i32, max: i32, flags: ImGuiSliderFlags) -> bool {
    unsafe { igSliderInt4(label.as_ptr(), v.as_mut_ptr(), min, max, core::ptr::null(), flags) }
}

pub fn slider_angle(label: &str, v_rad: &mut f32, min_degrees: f32, max_degrees: f32, flags: ImGuiSliderFlags) -> bool {
    unsafe { igSliderAngle(label.as_ptr(), v_rad as *mut f32, min_degrees, max_degrees, core::ptr::null(), flags) }
}

pub fn vslider_float(label: &str, size: ImVec2, v: &mut f32, min: f32, max: f32, flags: ImGuiSliderFlags) -> bool {
    unsafe { igVSliderFloat(label.as_ptr(), size, v as *mut f32, min, max, core::ptr::null(), flags) }
}

pub fn vslider_int(label: &str, size: ImVec2, v: &mut i32, min: i32, max: i32, flags: ImGuiSliderFlags) -> bool {
    unsafe { igVSliderInt(label.as_ptr(), size, v as *mut _, min, max, core::ptr::null(), flags) }
}
