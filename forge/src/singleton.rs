use alloc::{boxed::Box, ffi::CString, vec::Vec};

use log::error;
use sys::singleton::*;

use crate::mt::object::{MtObject, Object};

pub struct SingletonManager;

impl SingletonManager {
    pub fn get_by_name(name: &str) -> Option<&'static mut MtObject> {
        Self::get_by_name_typed(name)
    }

    pub fn get_by_id(id: u32) -> Option<&'static mut MtObject> {
        Self::get_by_id_typed(id)
    }

    pub fn get_by_name_typed<T: Object>(name: &str) -> Option<&'static mut T> {
        let c_name = CString::new(name).ok()?;
        let ptr = unsafe { forge_singleton_getInstanceByName(c_name.as_bytes().as_ptr()) };

        if ptr.is_null() {
            None
        } else {
            Some(unsafe { &mut *(ptr as *mut T) })
        }
    }

    pub fn get_by_id_typed<T: Object>(id: u32) -> Option<&'static mut T> {
        let ptr = unsafe { forge_singleton_getInstanceById(id) };

        if ptr.is_null() {
            None
        } else {
            Some(unsafe { &mut *(ptr as *mut T) })
        }
    }

    pub fn get_all() -> Box<[&'static mut MtObject]> {
        let count = unsafe { forge_singleton_getAllInstances(core::ptr::null_mut(), 0) };
        let mut instances = Vec::with_capacity(count as usize);

        let actual = unsafe { forge_singleton_getAllInstances(instances.as_mut_ptr(), count) };
        if actual != count {
            error!("Mismatching singleton count. Expected {count} got {actual}");
        }

        instances
            .iter()
            .map(|&inst| unsafe { &mut *(inst as *mut MtObject) })
            .collect()
    }
}
