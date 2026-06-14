use core::ffi::c_char;

unsafe extern "C" {
    pub fn forge_fs_mountSaveData() -> bool;
    pub fn forge_fs_unmountSaveData();
    pub fn forge_fs_getSaveDataMountPoint() -> *const c_char;
    pub fn forge_fs_getRomFsMountPoint() -> *const c_char;
}
