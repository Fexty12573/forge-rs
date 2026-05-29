use core::ffi::c_void;

unsafe extern "C" {
    pub fn forge_singleton_getInstanceByName(name: *const u8) -> *mut c_void;
    pub fn forge_singleton_getInstanceById(id: u32) -> *mut c_void;
    pub fn forge_singleton_getAllInstances(out: *mut *mut c_void, max: u32) -> u32;
}
