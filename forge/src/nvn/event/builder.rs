use sys::nvn::types::NVNeventBuilder;

use crate::nvn::owned::Owned;
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
            ffi::event_builder_set_storage(&mut self.0, pool.as_ptr(), offset);
        }
        self
    }

    pub fn build(&self) -> Option<Owned<Event>> {
        Owned::<Event>::try_new(|event| unsafe { ffi::event_initialize(event.as_ptr(), &self.0) })
    }
}
