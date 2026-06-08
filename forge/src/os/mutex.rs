use sys::os::mutex::*;

pub struct Mutex {
    inner: MutexType,
}

impl Mutex {
    pub fn new() -> Self {
        Self::new_internal(false)
    }

    pub fn recursive() -> Self {
        Self::new_internal(true)
    }

    pub fn finalize(&mut self) {
        unsafe { nnosFinalizeMutex(self.ptr()) };
    }

    pub fn lock(&mut self) {
        unsafe { nnosLockMutex(self.ptr()) };
    }

    pub fn try_lock(&mut self) -> bool {
        unsafe { nnosTryLockMutex(self.ptr()) }
    }

    pub fn unlock(&mut self) {
        unsafe { nnosUnlockMutex(self.ptr()) };
    }

    pub(crate) fn ptr(&mut self) -> *mut MutexType {
        &mut self.inner as *mut MutexType
    }

    fn new_internal(recursive: bool) -> Self {
        let mut inner = MutexType::default();
        unsafe { nnosInitializeMutex(&mut inner as *mut MutexType, recursive, 0) };
        Self { inner }
    }
}

impl Drop for Mutex {
    fn drop(&mut self) {
        todo!()
    }
}
