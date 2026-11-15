use core::{cell::UnsafeCell, time::Duration};

use sys::os::{TimeSpanType, condvar::*};

use crate::os::mutex::{MutexGuard, RawMutex};

/// A condition variable for blocking threads until a condition becomes true.
///
/// Always used together with a mutex: callers hold the lock while checking the condition, then pass
/// the guard to [`wait`](Self::wait) so the lock is released atomically while the thread sleeps.
/// The `*_raw` variants take a [`RawMutex`] instead.
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

    /// Atomically releases the lock held by `guard` and blocks until signaled, then reacquires it.
    pub fn wait<'a, T: ?Sized>(&self, guard: MutexGuard<'a, T>) -> MutexGuard<'a, T> {
        guard.release_owner();
        self.wait_raw(guard.raw());
        guard.reacquire_owner();
        guard
    }

    /// Like [`wait`](Self::wait), but returns after at most `timeout`. Also returns the wait status.
    pub fn wait_timeout<'a, T: ?Sized>(
        &self,
        guard: MutexGuard<'a, T>,
        timeout: Duration,
    ) -> (MutexGuard<'a, T>, ConditionVariableStatus) {
        guard.release_owner();
        let status = self.wait_timeout_raw(guard.raw(), timeout);
        guard.reacquire_owner();
        (guard, status)
    }

    /// Atomically releases `mutex` and blocks until signaled, then reacquires `mutex`.
    pub fn wait_raw(&self, mutex: &RawMutex) {
        unsafe { nnosWaitConditionVariable(self.ptr(), mutex.ptr()) };
    }

    /// Like [`wait_raw`](Self::wait_raw), but returns after at most `timeout`. Returns the wait status.
    pub fn wait_timeout_raw(&self, mutex: &RawMutex, timeout: Duration) -> ConditionVariableStatus {
        let timeout = u64::try_from(timeout.as_nanos()).unwrap_or(u64::MAX);
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
