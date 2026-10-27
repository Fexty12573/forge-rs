use sys::nvn::types::{NVNevent, NVNeventBuilder};

use crate::nvn::{event::Event, funcs as ffi, memory_pool::MemoryPool};

#[repr(transparent)]
pub struct EventBuilder(pub NVNeventBuilder);

impl EventBuilder {
    pub fn new() -> Self {
        let mut b = EventBuilder(NVNeventBuilder::default());
        unsafe {
            ffi::event_builder_set_defaults(&mut b.0);
        }

        b
    }

    pub fn set_storage(&mut self, pool: &MemoryPool, offset: i64) -> &mut Self {
        unsafe {
            ffi::event_builder_set_storage(&mut self.0, &pool.0, offset);
        }
        self
    }

    pub fn build(&self) -> Event {
        let mut event = NVNevent::default();
        unsafe {
            ffi::event_initialize(&mut event, &self.0);
        }

        Event(event)
    }
}
