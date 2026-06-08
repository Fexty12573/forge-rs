use core::time::Duration;

use sys::os::{EventClearMode, TimeSpanType, light_event::*};

pub struct LightEvent {
    inner: LightEventType,
}

impl LightEvent {
    pub const fn new(signaled: bool, clear_mode: EventClearMode) -> Self {
        Self {
            inner: LightEventType::new(signaled, clear_mode),
        }
    }

    pub fn finalize(&mut self) {
        unsafe { forge_nnosFinalizeLightEvent(self.ptr()) };
    }

    pub fn signal(&mut self) {
        unsafe { forge_nnosSignalLightEvent(self.ptr()) };
    }

    pub fn wait(&mut self) {
        unsafe { forge_nnosWaitLightEvent(self.ptr()) };
    }

    pub fn try_wait(&mut self) -> bool {
        unsafe { forge_nnosTryWaitLightEvent(self.ptr()) }
    }

    pub fn wait_timeout(&mut self, timeout: Duration) -> bool {
        let timeout = timeout.as_nanos() as u64;
        unsafe { forge_nnosTimedWaitLightEvent(self.ptr(), TimeSpanType(timeout)) }
    }

    pub fn clear(&mut self) {
        unsafe { forge_nnosClearLightEvent(self.ptr()) };
    }

    pub(crate) fn ptr(&mut self) -> *mut LightEventType {
        &mut self.inner
    }
}

impl Drop for LightEvent {
    fn drop(&mut self) {
        self.finalize();
    }
}
