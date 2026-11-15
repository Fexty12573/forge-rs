use crate::os::mutex::RawMutex;

/// An RAII guard that locks a [`RawMutex`] on creation and unlocks it on drop.
pub struct ScopedLock<'a> {
    mtx: &'a RawMutex,
}

impl<'a> ScopedLock<'a> {
    /// Locks `mtx` and returns a guard that unlocks it when dropped.
    pub fn new(mtx: &'a RawMutex) -> Self {
        mtx.lock();
        Self { mtx }
    }
}

impl<'a> Drop for ScopedLock<'a> {
    fn drop(&mut self) {
        self.mtx.unlock();
    }
}
