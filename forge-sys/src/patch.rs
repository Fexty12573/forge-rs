use core::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Patch {
    addr: u32,
    size: u32,
    patch_bytes: *const c_void,
    original_bytes: *const c_void,
    enabled: bool,
}

unsafe extern "C" {
    pub fn forge_patch_create(address: u32, bytes: *const c_void, length: u32, enable: bool) -> Patch;
    pub fn forge_patch_destroy(patch: *mut Patch);

    pub fn forge_patch_enable(patch: *mut Patch);
    pub fn forge_patch_disable(patch: *mut Patch);
}

impl Patch {
    pub fn addr(&self) -> u32 {
        self.addr
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn patch_bytes(&self) -> *const c_void {
        self.patch_bytes
    }

    pub fn original_bytes(&self) -> *const c_void {
        self.original_bytes
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }
}
