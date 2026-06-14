use core::ffi::CStr;
use sys::fs::*;

pub mod romfs {
    use super::*;

    /// Returns the mount point of the RomFS filesystem. Usually `app`.
    pub fn mount_point() -> &'static str {
        let ptr = unsafe { forge_fs_getRomFsMountPoint() };
        unsafe { CStr::from_ptr::<'static>(ptr) }
            .to_str()
            .expect("Invalid UTF-8 in mount point")
    }
}

pub mod savedata {
    use super::*;

    /// Returns the mount point of the SaveData filesystem. Usually `savedata`.
    pub fn mount_point() -> &'static str {
        let ptr = unsafe { forge_fs_getSaveDataMountPoint() };
        unsafe { CStr::from_ptr::<'static>(ptr) }
            .to_str()
            .expect("Invalid UTF-8 in mount point")
    }

    /// Mounts the SaveData filesystem. Returns `true` if successful, `false` otherwise.
    ///
    /// # Safety
    /// You must check the return value of this function before attempting to access the SaveData filesystem.
    pub fn mount() -> bool {
        unsafe { forge_fs_mountSaveData() }
    }

    /// Unmounts the SaveData filesystem.
    ///
    /// # Safety
    /// Do not call this function if any of the following conditions are true:
    /// - You have any open file handles to the SaveData filesystem.
    /// - The last call to `mount()` returned `false`.
    /// - You have not called `mount()` at least once before calling this function.
    pub fn unmount() {
        unsafe { forge_fs_unmountSaveData() }
    }
}
