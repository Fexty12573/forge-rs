use crate::os::mutex::Mutex;

/// An RAII guard that locks a [`Mutex`] on creation and unlocks it on drop.
pub struct ScopedLock<'a> {
    mtx: &'a Mutex,
}

impl<'a> ScopedLock<'a> {
    /// Locks `mtx` and returns a guard that unlocks it when dropped.
    pub fn new(mtx: &'a Mutex) -> Self {
        mtx.lock();
        Self { mtx }
    }
}

impl<'a> Drop for ScopedLock<'a> {
    fn drop(&mut self) {
        self.mtx.unlock();
    }
}
