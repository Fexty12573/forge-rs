use core::time::Duration;

use sys::os::{TimeSpanType, condvar::*};

use crate::os::mutex::Mutex;

pub struct ConditionVariable {
    inner: ConditionVariableType,
}

impl ConditionVariable {
    pub fn new() -> Self {
        let mut inner = ConditionVariableType::default();
        unsafe { nnosInitializeConditionVariable(&mut inner as *mut ConditionVariableType) };
        Self { inner }
    }

    pub fn finalize(&mut self) {
        unsafe { nnosFinalizeConditionVariable(self.ptr()) };
    }

    pub fn signal(&mut self) {
        unsafe { nnosSignalConditionVariable(self.ptr()) };
    }

    pub fn broadcast(&mut self) {
        unsafe { nnosBroadcastConditionVariable(self.ptr()) };
    }

    pub fn wait(&mut self, mutex: &mut Mutex) {
        unsafe { nnosWaitConditionVariable(self.ptr(), mutex.ptr()) };
    }

    pub fn wait_timeout(&mut self, mutex: &mut Mutex, timeout: Duration) -> ConditionVariableStatus {
        let timeout = timeout.as_nanos() as u64;
        unsafe { nnosTimedWaitConditionVariable(self.ptr(), mutex.ptr(), TimeSpanType(timeout)) }
    }

    pub(crate) fn ptr(&mut self) -> *mut ConditionVariableType {
        &mut self.inner as *mut ConditionVariableType
    }
}

impl Drop for ConditionVariable {
    fn drop(&mut self) {
        self.finalize();
    }
}
