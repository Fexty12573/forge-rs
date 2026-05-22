use core::ffi::{CStr, c_char, c_void};

use crate::mt::crc::MtCRC;

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

impl MtDti {
    pub fn make_id(name: &str) -> u32 {
        MtCRC::from_str(name, 0xFFFFFFFF) & 0x7FFFFFFF
    }

    pub fn find(name: &str) -> Option<&MtDti> {
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
}
