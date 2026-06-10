use core::{cell::UnsafeCell, time::Duration};

use sys::os::{TimeSpanType, event::*};

/// A kernel event object that threads can signal and wait on.
pub struct Event {
    inner: UnsafeCell<EventType>,
}

impl Event {
    /// Creates a new event. If `signaled` is `true` the event starts in the signaled state.
    pub fn new(signaled: bool, clear_mode: EventClearMode) -> Self {
        let mut inner = EventType::default();
        unsafe { nnosInitializeEvent(&mut inner, signaled, clear_mode) };
        Self {
            inner: UnsafeCell::new(inner),
        }
    }

    /// Destroys the event. Called automatically on drop.
    pub fn finalize(&self) {
        unsafe { nnosFinalizeEvent(self.ptr()) };
    }

    /// Signals the event, waking any threads currently blocked in [`wait`](Self::wait).
    pub fn signal(&self) {
        unsafe { nnosSignalEvent(self.ptr()) };
    }

    /// Blocks the calling thread until the event is signaled.
    pub fn wait(&self) {
        unsafe { nnosWaitEvent(self.ptr()) };
    }

    /// Returns `true` if the event is currently signaled without blocking.
    pub fn try_wait(&self) -> bool {
        unsafe { nnosTryWaitEvent(self.ptr()) }
    }

    /// Blocks until the event is signaled or `timeout` elapses. Returns `true` if signaled.
    pub fn wait_timeout(&self, timeout: Duration) -> bool {
        let timeout = timeout.as_nanos() as u64;
        unsafe { nnosTimedWaitEvent(self.ptr(), TimeSpanType(timeout)) }
    }

    /// Clears the event, resetting it to the unsignaled state.
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
