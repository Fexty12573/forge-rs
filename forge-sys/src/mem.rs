unsafe extern "C" {
    pub fn forge_mem_getMainTextAddr() -> u32;
    pub fn forge_mem_getMainRoDataAddr() -> u32;
    pub fn forge_mem_getMainDataAddr() -> u32;
    pub fn forge_mem_getMainBssAddr() -> u32;
    pub fn forge_mem_getMainHeapAddr() -> u32;

    pub fn malloc(size: usize) -> *mut core::ffi::c_void;
    pub fn realloc(ptr: *mut core::ffi::c_void, size: usize) -> *mut core::ffi::c_void;
    pub fn free(ptr: *mut core::ffi::c_void);
}
