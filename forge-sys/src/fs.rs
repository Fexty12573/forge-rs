use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn forge_fs_mountSaveData() -> bool;
    pub fn forge_fs_unmountSaveData();
    pub fn forge_fs_getSaveDataMountPoint() -> *const c_char;
    pub fn forge_fs_getRomFsMountPoint() -> *const c_char;

    pub fn forge_fs_fileExists(path: *const c_char) -> bool;
    pub fn forge_fs_openFile(path: *const c_char, mode: i32, out_handle: *mut u32) -> bool;
    pub fn forge_fs_getFileSize(handle: u32, out_size: *mut i64) -> bool;
    pub fn forge_fs_readFile(handle: u32, offset: i64, buffer: *mut c_void, size: usize) -> bool;
    pub fn forge_fs_closeFile(handle: u32);
}
