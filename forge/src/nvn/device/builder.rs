use sys::nvn::types::NVNdeviceBuilder;

use crate::nvn::owned::Owned;
use crate::nvn::{DeviceFlags_, device::Device, funcs as ffi};

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

    pub fn build(&self) -> Option<Owned<Device>> {
        Owned::<Device>::try_new(|device| unsafe { ffi::device_initialize(device.as_ptr(), &self.0) })
    }
}
