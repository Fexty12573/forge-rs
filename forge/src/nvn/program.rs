use core::cell::UnsafeCell;
use core::ffi::CStr;

use sys::nvn::types::NVNprogram;

use crate::nvn::owned::{Owned, impl_object};
use crate::nvn::{ShaderData, SubroutineLinkageMapPtr, device::Device, funcs as ffi};

#[repr(transparent)]
pub struct Program(UnsafeCell<NVNprogram>);

impl_object!(Program, NVNprogram, ffi::program_finalize);

impl Program {
    #[inline]
    pub fn as_ptr(&self) -> *mut NVNprogram {
        self.0.get()
    }

    pub fn new(device: &Device) -> Option<Owned<Self>> {
        Owned::<Self>::try_new(|program| unsafe { ffi::program_initialize(program.as_ptr(), device.as_ptr()) })
    }

    #[inline]
    pub fn set_debug_label(&mut self, label: &CStr) {
        unsafe {
            ffi::program_set_debug_label(self.as_ptr(), label.as_ptr());
        }
    }

    #[inline]
    pub fn set_shaders(&mut self, stages: &[ShaderData]) -> bool {
        unsafe { ffi::program_set_shaders(self.as_ptr(), stages.len() as i32, stages.as_ptr()) }
    }

    #[inline]
    pub fn set_subroutine_linkage(&mut self, linkage_maps: &[SubroutineLinkageMapPtr]) -> bool {
        unsafe { ffi::program_set_subroutine_linkage(self.as_ptr(), linkage_maps.len() as i32, linkage_maps.as_ptr()) }
    }
}
