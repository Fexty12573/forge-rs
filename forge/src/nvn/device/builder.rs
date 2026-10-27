use sys::nvn::types::NVNdeviceBuilder;

use crate::nvn::{DeviceFlags_, funcs as ffi};

#[repr(transparent)]
pub struct DeviceBuilder(pub NVNdeviceBuilder);

impl DeviceBuilder {
    pub fn new() -> Self {
        let mut b = DeviceBuilder(NVNdeviceBuilder::default());
        unsafe {
            ffi::device_builder_set_defaults(&mut b.0);
        }

        b
    }

    pub fn set_flags(&mut self, flags: DeviceFlags_) -> &mut Self {
        unsafe {
            ffi::device_builder_set_flags(&mut self.0, flags);
        }
        self
    }
}
