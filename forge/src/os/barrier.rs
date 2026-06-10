use core::cell::UnsafeCell;

use sys::os::barrier::*;

/// A synchronization barrier that blocks threads until a fixed number have arrived.
pub struct Barrier {
    inner: UnsafeCell<BarrierType>,
}

impl Barrier {
    /// Creates a new barrier that releases all waiting threads once `threads` have called [`wait`](Self::wait).
    pub fn new(threads: u32) -> Self {
        let mut inner = BarrierType::default();
        unsafe { nnosInitializeBarrier(&mut inner as *mut BarrierType, threads as i32) };
        Self {
            inner: UnsafeCell::new(inner),
        }
    }

    /// Destroys the barrier. Called automatically on drop.
    pub fn finalize(&self) {
        unsafe { nnosFinalizeBarrier(self.ptr()) };
    }

    /// Blocks the calling thread until all `threads` threads have called `wait`.
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
