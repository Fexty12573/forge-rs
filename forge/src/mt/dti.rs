use core::ffi::{CStr, c_char, c_void};

use macros::{HasVtable, pure_virtual};

use crate::mt::{crc::MtCRC, object::Object};

#[repr(C)]
#[derive(HasVtable)]
pub struct MtDti {
    _vft: *const c_void,
    name: *const c_char,
    next: *const MtDti,
    child: *const MtDti,
    parent: *const MtDti,
    link: *const MtDti,
    meta: u32,
    id: u32,
}

pub trait CacheDti {
    fn dti() -> Option<&'static MtDti>;
}

impl MtDti {
    pub fn make_id(name: &str) -> u32 {
        MtCRC::from_str(name, 0xFFFFFFFF) & 0x7FFFFFFF
    }

    pub fn find(name: &str) -> Option<&'static MtDti> {
        type FindDtiFunc = unsafe extern "C" fn(u32) -> *const MtDti;
        let addr = crate::mem::text_addr() + 0x7AEF68;
        let func: FindDtiFunc = unsafe { core::mem::transmute(addr as *const c_void) };

        let dti = unsafe { func(Self::make_id(name)) };
        if dti == core::ptr::null() {
            None
        } else {
            Some(unsafe { &*dti })
        }
    }

    pub fn new<T: Object>(&self) -> Option<&'static mut T> {
        unsafe {
            let ptr = self.new_instance_impl();
            if ptr != core::ptr::null_mut() {
                Some(&mut *(ptr as *mut T))
            } else {
                None
            }
        }
    }

    pub fn instantiate<T: Object>(&self, obj: &mut T) -> bool {
        let ptr = self.instantiate_impl(core::ptr::from_mut(obj) as *mut c_void);
        ptr != core::ptr::null_mut()
    }

    pub fn name(&self) -> &str {
        unsafe { CStr::from_ptr(self.name).to_str().unwrap() }
    }

    pub fn parent(&self) -> Option<&MtDti> {
        if self.parent.is_null() {
            None
        } else {
            Some(unsafe { &*self.parent })
        }
    }

    pub fn child(&self) -> Option<&MtDti> {
        if self.child.is_null() {
            None
        } else {
            Some(unsafe { &*self.child })
        }
    }

    pub fn next(&self) -> Option<&MtDti> {
        if self.next.is_null() {
            None
        } else {
            Some(unsafe { &*self.next })
        }
    }

    pub fn link(&self) -> Option<&MtDti> {
        if self.link.is_null() {
            None
        } else {
            Some(unsafe { &*self.link })
        }
    }

    pub fn size(&self) -> usize {
        ((self.meta & 0x7FFFFF) << 2) as usize
    }

    pub fn allocator_index(&self) -> usize {
        ((self.meta >> 23) & 0x3F) as usize
    }

    pub fn attr(&self) -> u32 {
        (self.meta >> 29) & 0x7
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn is_a(&self, other_id: u32) -> bool {
        let mut current = Some(self);
        while let Some(dti) = current {
            if dti.id == other_id {
                return true;
            }
            current = dti.parent();
        }
        false
    }

    pub fn is_a_dti(&self, other: &MtDti) -> bool {
        self.is_a(other.id)
    }

    pub fn is_a_str(&self, other_name: &str) -> bool {
        self.is_a(Self::make_id(other_name))
    }

    // Virtual Functions
    #[pure_virtual(0)]
    fn dtor(&mut self);

    #[pure_virtual(1)]
    fn dtor2(&mut self);

    #[pure_virtual(2)]
    fn new_instance_impl(&self) -> *mut c_void;

    #[pure_virtual(3)]
    fn instantiate_impl(&self, obj: *mut c_void) -> *mut c_void;

    #[pure_virtual(4)]
    fn instantiate_array_impl(&self, objs: *mut c_void, count: i64) -> *mut c_void;
}
