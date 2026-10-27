use core::ffi::CStr;

use sys::nvn::types::NVNprogram;

use crate::nvn::{ShaderData, SubroutineLinkageMapPtr, device::Device, funcs as ffi};

#[repr(transparent)]
pub struct Program(pub NVNprogram);

impl Program {
    pub fn new(device: &Device) -> Self {
        let mut program = NVNprogram::default();
        unsafe {
            ffi::program_initialize(&mut program, device.as_ptr());
        }

        Self(program)
    }

    #[inline]
    pub fn set_debug_label(&mut self, label: &CStr) {
        unsafe {
            ffi::program_set_debug_label(&mut self.0, label.as_ptr());
        }
    }

    #[inline]
    pub fn set_shaders(&mut self, stages: &[ShaderData]) -> bool {
        unsafe { ffi::program_set_shaders(&mut self.0, stages.len() as i32, stages.as_ptr()) }
    }

    #[inline]
    pub fn set_subroutine_linkage(&mut self, linkage_maps: &[SubroutineLinkageMapPtr]) -> bool {
        unsafe { ffi::program_set_subroutine_linkage(&mut self.0, linkage_maps.len() as i32, linkage_maps.as_ptr()) }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            ffi::program_finalize(&mut self.0);
        }
    }
}
