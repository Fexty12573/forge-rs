use core::{cell::UnsafeCell, time::Duration};

use sys::os::{EventClearMode, TimeSpanType, light_event::*};

pub struct LightEvent {
    inner: UnsafeCell<LightEventType>,
}

impl LightEvent {
    pub const fn new(signaled: bool, clear_mode: EventClearMode) -> Self {
        Self {
            inner: UnsafeCell::new(LightEventType::new(signaled, clear_mode)),
        }
    }

    pub fn finalize(&self) {
        unsafe { forge_nnosFinalizeLightEvent(self.ptr()) };
    }

    pub fn signal(&self) {
        unsafe { forge_nnosSignalLightEvent(self.ptr()) };
    }

    pub fn wait(&self) {
        unsafe { forge_nnosWaitLightEvent(self.ptr()) };
    }

    pub fn try_wait(&self) -> bool {
        unsafe { forge_nnosTryWaitLightEvent(self.ptr()) }
    }

    pub fn wait_timeout(&self, timeout: Duration) -> bool {
        let timeout = timeout.as_nanos() as u64;
        unsafe { forge_nnosTimedWaitLightEvent(self.ptr(), TimeSpanType(timeout)) }
    }

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
