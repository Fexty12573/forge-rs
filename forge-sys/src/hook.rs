use core::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Hook {
    pub target: *const c_void,
    pub detour: *const c_void,
    pub original: *const c_void,
    jit1: [u8; 24], // Opaque data for libnx JIT structure
    jit2: [u8; 24], // Opaque data for libnx JIT structure
    has_ctx: bool,
}

unsafe extern "C" {
    pub fn forge_hook_create(target: *const c_void, detour: *const c_void, original: *mut *const c_void) -> Hook;
    pub fn forge_hook_createWithContext(
        target: *const c_void,
        detour: *const c_void,
        original: *mut *const c_void,
        context: *const c_void,
    ) -> Hook;
    pub fn forge_hook_getContext() -> *const c_void;
    pub fn forge_hook_updateContext(hook: *mut Hook, new_ctx: *const c_void) -> u32;
}
