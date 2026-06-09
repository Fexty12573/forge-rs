use crate::os::mutex::Mutex;

pub struct ScopedLock<'a> {
    mtx: &'a Mutex,
}

impl<'a> ScopedLock<'a> {
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
