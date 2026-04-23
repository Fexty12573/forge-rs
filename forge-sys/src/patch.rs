use core::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Patch {
    pub addr: u32,
    pub size: u32,
    pub patch_bytes: *const c_void,
    pub original_bytes: *const c_void,
    pub enabled: bool,
}

unsafe extern "C" {
    pub fn forge_patch_create(address: u32, bytes: *const c_void, length: u32, enable: bool) -> Patch;
    pub fn forge_patch_destroy(patch: *mut Patch);

    pub fn forge_patch_enable(patch: *mut Patch);
    pub fn forge_patch_disable(patch: *mut Patch);
}
