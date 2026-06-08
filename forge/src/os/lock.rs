use crate::os::mutex::Mutex;

pub struct ScopedLock<'a> {
    mtx: &'a mut Mutex,
}

impl<'a> ScopedLock<'a> {
    pub fn new(mtx: &'a mut Mutex) -> Self {
        mtx.lock();
        Self { mtx }
    }
}

impl<'a> Drop for ScopedLock<'a> {
    fn drop(&mut self) {
        self.mtx.unlock();
    }
}
