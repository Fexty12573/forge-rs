use alloc::{boxed::Box, ffi::CString, vec};
use core::ffi::CStr;
use log::error;
use sys::fs::*;

pub fn file_exists(path: &str) -> bool {
    match CString::new(path) {
        Ok(path) => unsafe { forge_fs_fileExists(path.as_ptr()) },
        Err(err) => {
            error!("{err}");
            false
        }
    }
}

/// Returns the size of the file at the given `path` as `Some(size)`
/// if it exists, otherwise `None`.
pub fn file_size(path: &str) -> Option<usize> {
    File::open(path, OpenMode::Read)?.size()
}

/// Reads the file at the given `path` into an array of bytes.
/// Returns `None` if any operation fails or the file doesn't exist.
pub fn read(path: &str) -> Option<Box<[u8]>> {
    File::open(path, OpenMode::Read)?.read()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum OpenMode {
    Read = 0x1,
    Write = 0x2,
    Append = 0x4,
    ReadWrite = 0x1 | 0x2,
}

pub struct File {
    handle: u32,
}

impl File {
    pub fn open(path: &str, mode: OpenMode) -> Option<File> {
        let path = CString::new(path).ok()?;
        let mut handle = 0;
        if !unsafe { forge_fs_openFile(path.as_ptr(), mode as i32, &mut handle) } {
            return None;
        }

        Some(File { handle })
    }

    pub fn size(&self) -> Option<usize> {
        let mut size = 0;
        unsafe { forge_fs_getFileSize(self.handle, &mut size) }.then_some(size as usize)
    }

    pub fn read(&self) -> Option<Box<[u8]>> {
        let mut buf = vec![0u8; self.size()?];
        unsafe { forge_fs_readFile(self.handle, 0, buf.as_mut_ptr().cast(), buf.len()) }.then_some(buf.into_boxed_slice())
    }

    pub fn read_into(&self, offset: usize, buffer: &mut [u8]) -> bool {
        unsafe { forge_fs_readFile(self.handle, offset as i64, buffer.as_mut_ptr().cast(), buffer.len()) }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe {
            forge_fs_closeFile(self.handle);
        }
    }
}

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

    /// Mounts SaveData for the lifespan of this object.
    ///
    /// # Panic
    /// Panics if SaveData fails to be mounted.
    pub struct ScopedMount;

    impl ScopedMount {
        pub fn new() -> Self {
            if !savedata::mount() {
                panic!("Failed to mount savedata");
            }

            Self
        }
    }

    impl Drop for ScopedMount {
        fn drop(&mut self) {
            savedata::unmount();
        }
    }
}
