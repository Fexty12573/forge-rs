use core::{cell::UnsafeCell, time::Duration};

use sys::os::{TimeSpanType, event::*};

pub struct Event {
    inner: UnsafeCell<EventType>,
}

impl Event {
    pub fn new(signaled: bool, clear_mode: EventClearMode) -> Self {
        let mut inner = EventType::default();
        unsafe { nnosInitializeEvent(&mut inner, signaled, clear_mode) };
        Self {
            inner: UnsafeCell::new(inner),
        }
    }

    pub fn finalize(&self) {
        unsafe { nnosFinalizeEvent(self.ptr()) };
    }

    pub fn signal(&self) {
        unsafe { nnosSignalEvent(self.ptr()) };
    }

    pub fn wait(&self) {
        unsafe { nnosWaitEvent(self.ptr()) };
    }

    pub fn try_wait(&self) -> bool {
        unsafe { nnosTryWaitEvent(self.ptr()) }
    }

    pub fn wait_timeout(&self, timeout: Duration) -> bool {
        let timeout = timeout.as_nanos() as u64;
        unsafe { nnosTimedWaitEvent(self.ptr(), TimeSpanType(timeout)) }
    }

    pub fn clear(&self) {
        unsafe { nnosClearEvent(self.ptr()) };
    }

    pub(crate) fn ptr(&self) -> *mut EventType {
        self.inner.get()
    }
}

impl Drop for Event {
    fn drop(&mut self) {
        self.finalize();
    }
}
