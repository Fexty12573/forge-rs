use core::ffi::{CStr, c_void};

use crate::mt::{crc::MtCRC, datatype::MtType, object::MtObject};

#[repr(C)]
pub struct MtProperty {
    name: *const u8,
    dtype: MtType,
    attr: u16,
    owner: *mut MtObject,
    get: *mut c_void,
    _unk1: *const c_void,
    count: u32,
    _unk2: *const c_void,
    set: *const c_void,
    _unk3: *const c_void,
    set_count: *const c_void,
    _unk4: *const c_void,
    index: u32,
    pub prev: *const MtProperty,
    pub next: *const MtProperty,
}

const ATTR_GET_SET: u16 = 0x80;
const ATTR_ARRAY: u16 = 0x20;
const ATTR_EVENT: u16 = 0x08;

impl MtProperty {
    pub fn name(&self) -> &str {
        unsafe { CStr::from_ptr(self.name) }
            .to_str()
            .expect("Failed to convert string to UTF-8")
    }

    pub fn has_name(&self) -> bool {
        !self.name.is_null()
    }

    pub fn dtype(&self) -> MtType {
        self.dtype
    }

    pub fn attr(&self) -> u16 {
        self.attr
    }

    pub fn crc(&self) -> u32 {
        MtCRC::from_str(self.name(), !0)
    }

    pub fn owner(&self) -> &mut MtObject {
        unsafe { &mut *self.owner }
    }

    pub fn owner_ptr(&self) -> *mut MtObject {
        self.owner
    }

    pub fn is_get_set(&self) -> bool {
        (self.attr & ATTR_GET_SET) != 0
    }

    pub fn is_array(&self) -> bool {
        (self.attr & ATTR_ARRAY) != 0
    }

    pub fn is_event(&self) -> bool {
        (self.attr & ATTR_EVENT) != 0 || matches!(self.dtype, MtType::Event | MtType::Event32 | MtType::Event64)
    }

    pub fn ptr<T>(&self) -> *mut T {
        self.get as *mut T
    }

    pub fn as_ref<T>(&self) -> &mut T {
        unsafe { &mut *(self.get as *mut T) }
    }

    pub fn get(&self) -> *const c_void {
        self.get
    }

    pub fn set(&self) -> *const c_void {
        self.set
    }

    pub fn getcount(&self) -> *const c_void {
        self.count as *const c_void
    }

    pub fn setcount(&self) -> *const c_void {
        self.set_count
    }

    pub fn count(&self) -> u32 {
        self.count
    }

    pub fn offset(&self) -> usize {
        (self.get as usize).saturating_sub(self.owner as usize)
    }

    pub fn next(&self) -> Option<&MtProperty> {
        if self.next.is_null() {
            None
        } else {
            Some(unsafe { &*self.next })
        }
    }

    pub fn prev(&self) -> Option<&MtProperty> {
        if self.prev.is_null() {
            None
        } else {
            Some(unsafe { &*self.prev })
        }
    }
}
