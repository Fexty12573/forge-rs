use core::ffi::CStr;

use sys::imgui::*;

pub fn input_text(label: &CStr, buf: &mut [u8], flags: ImGuiInputTextFlags) -> bool {
    unsafe {
        igInputText(
            label.as_ptr(),
            buf.as_mut_ptr() as *mut _,
            buf.len(),
            flags,
            None,
            core::ptr::null_mut(),
        )
    }
}

pub fn input_text_multiline(label: &CStr, buf: &mut [u8], size: ImVec2, flags: ImGuiInputTextFlags) -> bool {
    unsafe {
        igInputTextMultiline(
            label.as_ptr(),
            buf.as_mut_ptr() as *mut _,
            buf.len(),
            size,
            flags,
            None,
            core::ptr::null_mut(),
        )
    }
}

pub fn input_text_with_hint(label: &CStr, hint: &CStr, buf: &mut [u8], flags: ImGuiInputTextFlags) -> bool {
    unsafe {
        igInputTextWithHint(
            label.as_ptr(),
            hint.as_ptr(),
            buf.as_mut_ptr() as *mut _,
            buf.len(),
            flags,
            None,
            core::ptr::null_mut(),
        )
    }
}

pub fn input_float(label: &CStr, v: &mut f32, step: f32, step_fast: f32, flags: ImGuiInputTextFlags) -> bool {
    unsafe { igInputFloat(label.as_ptr(), v as *mut f32, step, step_fast, b"%.3f\0".as_ptr(), flags) }
}

pub fn input_float2(label: &CStr, v: &mut [f32; 2], flags: ImGuiInputTextFlags) -> bool {
    unsafe { igInputFloat2(label.as_ptr(), v.as_mut_ptr(), b"%.3f\0".as_ptr(), flags) }
}

pub fn input_float3(label: &CStr, v: &mut [f32; 3], flags: ImGuiInputTextFlags) -> bool {
    unsafe { igInputFloat3(label.as_ptr(), v.as_mut_ptr(), b"%.3f\0".as_ptr(), flags) }
}

pub fn input_float4(label: &CStr, v: &mut [f32; 4], flags: ImGuiInputTextFlags) -> bool {
    unsafe { igInputFloat4(label.as_ptr(), v.as_mut_ptr(), b"%.3f\0".as_ptr(), flags) }
}

pub fn input_int(label: &CStr, v: &mut i32, step: i32, step_fast: i32, flags: ImGuiInputTextFlags) -> bool {
    unsafe { igInputInt(label.as_ptr(), v as *mut _, step, step_fast, flags) }
}

pub fn input_int2(label: &CStr, v: &mut [i32; 2], flags: ImGuiInputTextFlags) -> bool {
    unsafe { igInputInt2(label.as_ptr(), v.as_mut_ptr(), flags) }
}

pub fn input_int3(label: &CStr, v: &mut [i32; 3], flags: ImGuiInputTextFlags) -> bool {
    unsafe { igInputInt3(label.as_ptr(), v.as_mut_ptr(), flags) }
}

pub fn input_int4(label: &CStr, v: &mut [i32; 4], flags: ImGuiInputTextFlags) -> bool {
    unsafe { igInputInt4(label.as_ptr(), v.as_mut_ptr(), flags) }
}

pub fn input_double(label: &CStr, v: &mut f64, step: f64, step_fast: f64, flags: ImGuiInputTextFlags) -> bool {
    unsafe { igInputDouble(label.as_ptr(), v as *mut f64, step, step_fast, b"%.6f\0".as_ptr(), flags) }
}
