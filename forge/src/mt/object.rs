use core::{ffi::c_void, mem::transmute};

use macros::Object;
use sys::cpp::HasVtable;

#[derive(Object)]
pub struct MtObject;

pub trait Object: HasVtable {
    fn dtor(&mut self) {
        let func: unsafe extern "C" fn(&mut Self) = unsafe { transmute(self.vtable_ptr().add(0)) };
        unsafe { func(self) }
    }

    fn destroy(&mut self, deallocate: bool) {
        let func: unsafe extern "C" fn(&mut Self, bool) = unsafe { transmute(self.vtable_ptr().add(1)) };
        unsafe { func(self, deallocate) }
    }

    fn create_ui(&self) {
        let func: unsafe extern "C" fn(&Self) = unsafe { transmute(self.vtable_ptr().add(2)) };
        unsafe { func(self) }
    }

    fn is_enable_instance(&self) -> bool {
        let func: unsafe extern "C" fn(&Self) -> bool = unsafe { transmute(self.vtable_ptr().add(3)) };
        unsafe { func(self) }
    }

    fn create_property(&self, props: *const c_void) {
        let func: unsafe extern "C" fn(&Self, *const c_void) = unsafe { transmute(self.vtable_ptr().add(4)) };
        unsafe { func(self, props) }
    }

    fn get_dti(&self) -> *const c_void {
        let func: unsafe extern "C" fn(&Self) -> *const c_void = unsafe { transmute(self.vtable_ptr().add(5)) };
        unsafe { func(self) }
    }
}
