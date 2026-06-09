use core::cell::UnsafeCell;

use sys::os::barrier::*;

pub struct Barrier {
    inner: UnsafeCell<BarrierType>,
}

impl Barrier {
    pub fn new(threads: u32) -> Self {
        let mut inner = BarrierType::default();
        unsafe { nnosInitializeBarrier(&mut inner as *mut BarrierType, threads as i32) };
        Self {
            inner: UnsafeCell::new(inner),
        }
    }

    pub fn finalize(&self) {
        unsafe { nnosFinalizeBarrier(self.ptr()) };
    }

    pub fn wait(&self) {
        unsafe { nnosAwaitBarrier(self.ptr()) };
    }

    pub(crate) fn ptr(&self) -> *mut BarrierType {
        self.inner.get()
    }
}

impl Drop for Barrier {
    fn drop(&mut self) {
        self.finalize();
    }
}
