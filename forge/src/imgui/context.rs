use core::ffi::CStr;

use sys::imgui::{
    ImGuiContext, ImGuiIO, ImGuiMemAllocFunc, ImGuiMemFreeFunc, ImGuiStyle, igGetCurrentContext, igGetIO_Nil, igGetStyle,
    igGetVersion, igSetAllocatorFunctions, igSetCurrentContext,
};

pub fn get_current_context() -> &'static mut ImGuiContext {
    unsafe { &mut *igGetCurrentContext() }
}

pub fn set_current_context(ctx: &'static mut ImGuiContext) {
    unsafe { igSetCurrentContext(ctx as *mut ImGuiContext) };
}

pub fn set_allocator_functions(alloc: ImGuiMemAllocFunc, free: ImGuiMemFreeFunc) {
    unsafe { igSetAllocatorFunctions(alloc, free, core::ptr::null_mut()) };
}

pub fn get_io() -> &'static mut ImGuiIO {
    unsafe { &mut *igGetIO_Nil() }
}

pub fn get_style() -> &'static mut ImGuiStyle {
    unsafe { &mut *igGetStyle() }
}

pub fn get_version() -> &'static str {
    unsafe { CStr::from_ptr(igGetVersion()).to_str().unwrap() }
}
