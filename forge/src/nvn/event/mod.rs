use sys::nvn::types::NVNevent;

mod builder;

pub use builder::EventBuilder;

use crate::nvn::{EventSignalMode, funcs as ffi};

#[repr(transparent)]
pub struct Event(pub NVNevent);

impl Event {
    #[inline]
    pub fn get_value(&self) -> u32 {
        unsafe { ffi::event_get_value(&self.0) }
    }

    #[inline]
    pub fn signal(&mut self, mode: EventSignalMode, value: u32) {
        unsafe {
            ffi::event_signal(&mut self.0, mode, value);
        }
    }
}

impl Drop for Event {
    fn drop(&mut self) {
        unsafe {
            ffi::event_finalize(&mut self.0);
        }
    }
}
