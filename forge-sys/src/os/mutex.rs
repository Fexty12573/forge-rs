#[repr(C)]
#[derive(Default)]
pub struct MutexType {
    _reserved: [u8; 20],
}

unsafe extern "C" {
    pub fn nnosInitializeMutex(mutex: *mut MutexType, recursive: bool, lock_level: i32);
    pub fn nnosFinalizeMutex(mutex: *mut MutexType);
    pub fn nnosLockMutex(mutex: *mut MutexType);
    pub fn nnosTryLockMutex(mutex: *mut MutexType) -> bool;
    pub fn nnosUnlockMutex(mutex: *mut MutexType);
}
