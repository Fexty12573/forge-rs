use core::cell::UnsafeCell;

use sys::os::mutex::*;

pub struct Mutex {
    inner: UnsafeCell<MutexType>,
}

impl Mutex {
    pub fn new() -> Self {
        Self::new_internal(false)
    }

    pub fn recursive() -> Self {
        Self::new_internal(true)
    }

    pub fn finalize(&self) {
        unsafe { nnosFinalizeMutex(self.ptr()) };
    }

    pub fn lock(&self) {
        unsafe { nnosLockMutex(self.ptr()) };
    }

    pub fn try_lock(&self) -> bool {
        unsafe { nnosTryLockMutex(self.ptr()) }
    }

    pub fn unlock(&self) {
        unsafe { nnosUnlockMutex(self.ptr()) };
    }

    pub(crate) fn ptr(&self) -> *mut MutexType {
        self.inner.get()
    }

    fn new_internal(recursive: bool) -> Self {
        let mut inner = MutexType::default();
        unsafe { nnosInitializeMutex(&mut inner as *mut MutexType, recursive, 0) };
        Self {
            inner: UnsafeCell::new(inner),
        }
    }
}

impl Drop for Mutex {
    fn drop(&mut self) {
        todo!()
    }
}
