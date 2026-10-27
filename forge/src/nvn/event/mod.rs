use core::cell::UnsafeCell;
use sys::nvn::types::NVNevent;

mod builder;

pub use builder::EventBuilder;

use crate::nvn::owned::impl_object;
use crate::nvn::{EventSignalMode, funcs as ffi};

#[repr(transparent)]
pub struct Event(UnsafeCell<NVNevent>);

impl_object!(Event, NVNevent, ffi::event_finalize);

impl Event {
    #[inline]
    pub fn as_ptr(&self) -> *mut NVNevent {
        self.0.get()
    }

    #[inline]
    pub fn get_value(&self) -> u32 {
        unsafe { ffi::event_get_value(self.as_ptr()) }
    }

    #[inline]
    pub fn signal(&mut self, mode: EventSignalMode, value: u32) {
        unsafe {
            ffi::event_signal(self.as_ptr(), mode, value);
        }
    }
}
