use core::{ffi::c_void, mem::transmute};

use macros::Object;
use sys::cpp::HasVtable;

use crate::mt::{dti::MtDti, property_list::MtPropertyList};

#[derive(Object)]
pub struct MtObject;

impl MtObject {
    pub fn dti(&self) -> &MtDti {
        unsafe { &*(self.get_dti() as *const MtDti) }
    }

    pub fn get_properties(&self) -> MtPropertyList {
        let mut props = MtPropertyList::new();
        self.create_property(&mut props as *mut MtPropertyList as *mut c_void);
        props
    }
}

pub trait Object: HasVtable {
    /// Runs the destructor on `self`
    fn dtor(&mut self) {
        let func: unsafe extern "C" fn(&mut Self) = unsafe { transmute(self.get_virtual_function(0)) };
        unsafe { func(self) }
    }

    /// Runs the destructor on `self` and deallocates the object
    fn destroy(&mut self) {
        let func: unsafe extern "C" fn(&mut Self) = unsafe { transmute(self.get_virtual_function(1)) };
        unsafe { func(self) }
    }

    fn create_ui(&self) {
        let func: unsafe extern "C" fn(&Self) = unsafe { transmute(self.get_virtual_function(2)) };
        unsafe { func(self) }
    }

    fn is_enable_instance(&self) -> bool {
        let func: unsafe extern "C" fn(&Self) -> bool = unsafe { transmute(self.get_virtual_function(3)) };
        unsafe { func(self) }
    }

    fn create_property(&self, props: *mut c_void) {
        let func: unsafe extern "C" fn(&Self, *mut c_void) = unsafe { transmute(self.get_virtual_function(4)) };
        unsafe { func(self, props) }
    }

    fn get_dti(&self) -> *const c_void {
        let func: unsafe extern "C" fn(&Self) -> *const c_void = unsafe { transmute(self.get_virtual_function(5)) };
        unsafe { func(self) }
    }
}
