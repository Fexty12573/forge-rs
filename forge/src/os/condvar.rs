use core::{cell::UnsafeCell, time::Duration};

use sys::os::{TimeSpanType, condvar::*};

use crate::os::mutex::Mutex;

/// A condition variable for blocking threads until a condition becomes true.
///
/// Always used together with a [`Mutex`]: callers hold the mutex while checking
/// the condition, then pass it to [`wait`](Self::wait) so the lock is released
/// atomically while the thread sleeps.
pub struct ConditionVariable {
    inner: UnsafeCell<ConditionVariableType>,
}

impl ConditionVariable {
    /// Creates a new condition variable.
    pub fn new() -> Self {
        let mut inner = ConditionVariableType::default();
        unsafe { nnosInitializeConditionVariable(&mut inner as *mut ConditionVariableType) };
        Self {
            inner: UnsafeCell::new(inner),
        }
    }

    /// Destroys the condition variable. Called automatically on drop.
    pub fn finalize(&self) {
        unsafe { nnosFinalizeConditionVariable(self.ptr()) };
    }

    /// Wakes one thread waiting on this condition variable.
    pub fn signal(&self) {
        unsafe { nnosSignalConditionVariable(self.ptr()) };
    }

    /// Wakes all threads waiting on this condition variable.
    pub fn broadcast(&self) {
        unsafe { nnosBroadcastConditionVariable(self.ptr()) };
    }

    /// Atomically releases `mutex` and blocks until signaled, then reacquires `mutex`.
    pub fn wait(&self, mutex: &Mutex) {
        unsafe { nnosWaitConditionVariable(self.ptr(), mutex.ptr()) };
    }

    /// Like [`wait`](Self::wait), but returns after at most `timeout`. Returns the wait status.
    pub fn wait_timeout(&self, mutex: &Mutex, timeout: Duration) -> ConditionVariableStatus {
        let timeout = timeout.as_nanos() as u64;
        unsafe { nnosTimedWaitConditionVariable(self.ptr(), mutex.ptr(), TimeSpanType(timeout)) }
    }

    pub(crate) fn ptr(&self) -> *mut ConditionVariableType {
        self.inner.get()
    }
}

impl Drop for ConditionVariable {
    fn drop(&mut self) {
        self.finalize();
    }
}
