use core::cell::UnsafeCell;

use sys::os::mutex::*;

/// A mutual exclusion primitive for protecting shared data.
pub struct Mutex {
    inner: UnsafeCell<MutexType>,
}

impl Mutex {
    /// Creates a non-recursive mutex.
    pub fn new() -> Self {
        Self::new_internal(false)
    }

    /// Creates a recursive mutex that can be locked multiple times by the same thread.
    pub fn recursive() -> Self {
        Self::new_internal(true)
    }

    /// Destroys the mutex. Called automatically on drop.
    pub fn finalize(&self) {
        unsafe { nnosFinalizeMutex(self.ptr()) };
    }

    /// Acquires the mutex, blocking until it is available.
    pub fn lock(&self) {
        unsafe { nnosLockMutex(self.ptr()) };
    }

    /// Attempts to acquire the mutex without blocking. Returns `true` on success.
    pub fn try_lock(&self) -> bool {
        unsafe { nnosTryLockMutex(self.ptr()) }
    }

    /// Releases the mutex.
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
