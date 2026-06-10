use core::{cell::UnsafeCell, time::Duration};

use sys::os::{EventClearMode, TimeSpanType, light_event::*};

/// A lightweight event object with the same API as [`Event`](super::event::Event) but suitable for `const` initialization.
pub struct LightEvent {
    inner: UnsafeCell<LightEventType>,
}

impl LightEvent {
    /// Creates a new light event. If `signaled` is `true` the event starts in the signaled state.
    pub const fn new(signaled: bool, clear_mode: EventClearMode) -> Self {
        Self {
            inner: UnsafeCell::new(LightEventType::new(signaled, clear_mode)),
        }
    }

    /// Destroys the event. Called automatically on drop.
    pub fn finalize(&self) {
        unsafe { forge_nnosFinalizeLightEvent(self.ptr()) };
    }

    /// Signals the event, waking any threads currently blocked in [`wait`](Self::wait).
    pub fn signal(&self) {
        unsafe { forge_nnosSignalLightEvent(self.ptr()) };
    }

    /// Blocks the calling thread until the event is signaled.
    pub fn wait(&self) {
        unsafe { forge_nnosWaitLightEvent(self.ptr()) };
    }

    /// Returns `true` if the event is currently signaled without blocking.
    pub fn try_wait(&self) -> bool {
        unsafe { forge_nnosTryWaitLightEvent(self.ptr()) }
    }

    /// Blocks until the event is signaled or `timeout` elapses. Returns `true` if signaled.
    pub fn wait_timeout(&self, timeout: Duration) -> bool {
        let timeout = timeout.as_nanos() as u64;
        unsafe { forge_nnosTimedWaitLightEvent(self.ptr(), TimeSpanType(timeout)) }
    }

    /// Clears the event, resetting it to the unsignaled state.
    pub fn clear(&self) {
        unsafe { forge_nnosClearLightEvent(self.ptr()) };
    }

    pub(crate) fn ptr(&self) -> *mut LightEventType {
        self.inner.get()
    }
}

impl Drop for LightEvent {
    fn drop(&mut self) {
        self.finalize();
    }
}
