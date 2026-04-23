use core::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Hook {
    pub target: *const c_void,
    pub detour: *const c_void,
    pub original: *const c_void,
    jit: [u8; 24], // Opaque data for libnx JIT structure
}

unsafe extern "C" {
    pub fn forge_hook_create(target: *const c_void, detour: *const c_void, original: *mut *const c_void) -> Hook;
}
