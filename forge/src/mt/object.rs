use core::ffi::c_void;

use crate::HasVtable;
use macros::pure_virtual;

#[derive(HasVtable)]
pub struct MtObject;

impl MtObject {
    #[pure_virtual(0)]
    pub fn dtor(&mut self) {}

    #[pure_virtual(1)]
    pub fn destroy(&mut self) {}

    #[pure_virtual(2)]
    pub fn create_ui(&self) {}

    #[pure_virtual(3)]
    pub fn is_enable_instance(&self) -> bool {}

    #[pure_virtual(4)]
    pub fn create_property(&self, props: *const c_void) {}

    #[pure_virtual(5)]
    pub fn get_dti(&self) -> *const c_void {}
}
