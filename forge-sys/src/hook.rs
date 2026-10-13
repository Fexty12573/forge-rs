use core::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Hook {
    pub target: *const c_void,
    pub detour: *const c_void,
    pub original: *const c_void,
    pub context: *const c_void,
    pub ctx_cell: *const c_void,
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
