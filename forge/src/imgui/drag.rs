use sys::imgui::*;

pub fn drag_float(label: &str, v: &mut f32, speed: f32, min: f32, max: f32, flags: ImGuiSliderFlags) -> bool {
    unsafe { igDragFloat(label.as_ptr(), v as *mut f32, speed, min, max, core::ptr::null(), flags) }
}

pub fn drag_float2(label: &str, v: &mut [f32; 2], speed: f32, min: f32, max: f32, flags: ImGuiSliderFlags) -> bool {
    unsafe { igDragFloat2(label.as_ptr(), v.as_mut_ptr(), speed, min, max, core::ptr::null(), flags) }
}

pub fn drag_float3(label: &str, v: &mut [f32; 3], speed: f32, min: f32, max: f32, flags: ImGuiSliderFlags) -> bool {
    unsafe { igDragFloat3(label.as_ptr(), v.as_mut_ptr(), speed, min, max, core::ptr::null(), flags) }
}

pub fn drag_float4(label: &str, v: &mut [f32; 4], speed: f32, min: f32, max: f32, flags: ImGuiSliderFlags) -> bool {
    unsafe { igDragFloat4(label.as_ptr(), v.as_mut_ptr(), speed, min, max, core::ptr::null(), flags) }
}

pub fn drag_float_range(label: &str, v_min: &mut f32, v_max: &mut f32, speed: f32, min: f32, max: f32, flags: ImGuiSliderFlags) -> bool {
    unsafe {
        igDragFloatRange2(
            label.as_ptr(),
            v_min as *mut f32,
            v_max as *mut f32,
            speed,
            min,
            max,
            core::ptr::null(),
            core::ptr::null(),
            flags,
        )
    }
}

pub fn drag_int(label: &str, v: &mut i32, speed: f32, min: i32, max: i32, flags: ImGuiSliderFlags) -> bool {
    unsafe { igDragInt(label.as_ptr(), v as *mut _, speed, min, max, core::ptr::null(), flags) }
}

pub fn drag_int2(label: &str, v: &mut [i32; 2], speed: f32, min: i32, max: i32, flags: ImGuiSliderFlags) -> bool {
    unsafe { igDragInt2(label.as_ptr(), v.as_mut_ptr(), speed, min, max, core::ptr::null(), flags) }
}

pub fn drag_int3(label: &str, v: &mut [i32; 3], speed: f32, min: i32, max: i32, flags: ImGuiSliderFlags) -> bool {
    unsafe { igDragInt3(label.as_ptr(), v.as_mut_ptr(), speed, min, max, core::ptr::null(), flags) }
}

pub fn drag_int4(label: &str, v: &mut [i32; 4], speed: f32, min: i32, max: i32, flags: ImGuiSliderFlags) -> bool {
    unsafe { igDragInt4(label.as_ptr(), v.as_mut_ptr(), speed, min, max, core::ptr::null(), flags) }
}

pub fn drag_int_range(label: &str, v_min: &mut i32, v_max: &mut i32, speed: f32, min: i32, max: i32, flags: ImGuiSliderFlags) -> bool {
    unsafe {
        igDragIntRange2(
            label.as_ptr(),
            v_min as *mut _,
            v_max as *mut _,
            speed,
            min,
            max,
            core::ptr::null(),
            core::ptr::null(),
            flags,
        )
    }
}
