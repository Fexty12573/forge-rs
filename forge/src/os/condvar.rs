use core::{cell::UnsafeCell, time::Duration};

use sys::os::{TimeSpanType, condvar::*};

use crate::os::mutex::Mutex;

pub struct ConditionVariable {
    inner: UnsafeCell<ConditionVariableType>,
}

impl ConditionVariable {
    pub fn new() -> Self {
        let mut inner = ConditionVariableType::default();
        unsafe { nnosInitializeConditionVariable(&mut inner as *mut ConditionVariableType) };
        Self {
            inner: UnsafeCell::new(inner),
        }
    }

    pub fn finalize(&self) {
        unsafe { nnosFinalizeConditionVariable(self.ptr()) };
    }

    pub fn signal(&self) {
        unsafe { nnosSignalConditionVariable(self.ptr()) };
    }

    pub fn broadcast(&self) {
        unsafe { nnosBroadcastConditionVariable(self.ptr()) };
    }

    pub fn wait(&self, mutex: &Mutex) {
        unsafe { nnosWaitConditionVariable(self.ptr(), mutex.ptr()) };
    }

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
