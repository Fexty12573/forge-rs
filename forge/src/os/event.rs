use core::time::Duration;

use sys::os::{TimeSpanType, event::*};

pub struct Event {
    inner: EventType,
}

impl Event {
    pub fn new(signaled: bool, clear_mode: EventClearMode) -> Self {
        let mut inner = EventType::default();
        unsafe { nnosInitializeEvent(&mut inner, signaled, clear_mode) };
        Self { inner }
    }

    pub fn finalize(&mut self) {
        unsafe { nnosFinalizeEvent(self.ptr()) };
    }

    pub fn signal(&mut self) {
        unsafe { nnosSignalEvent(self.ptr()) };
    }

    pub fn wait(&mut self) {
        unsafe { nnosWaitEvent(self.ptr()) };
    }

    pub fn try_wait(&mut self) -> bool {
        unsafe { nnosTryWaitEvent(self.ptr()) }
    }

    pub fn wait_timeout(&mut self, timeout: Duration) -> bool {
        let timeout = timeout.as_nanos() as u64;
        unsafe { nnosTimedWaitEvent(self.ptr(), TimeSpanType(timeout)) }
    }

    pub fn clear(&mut self) {
        unsafe { nnosClearEvent(self.ptr()) };
    }

    pub(crate) fn ptr(&mut self) -> *mut EventType {
        &mut self.inner
    }
}

impl Drop for Event {
    fn drop(&mut self) {
        self.finalize();
    }
}
