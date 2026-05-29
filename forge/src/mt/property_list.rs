use core::{ffi::c_void, marker::PhantomData};

use macros::Object;

use crate::{mem, mt::property::MtProperty};

#[repr(C)]
#[derive(Object)]
pub struct MtPropertyList {
    _vft: *const c_void,
    first: *const MtProperty,
}

pub struct MtPropertyListIter<'a> {
    current: *const MtProperty,
    _phantom: PhantomData<&'a MtProperty>,
}

impl MtPropertyList {
    pub fn new() -> Self {
        Self {
            _vft: (mem::text_addr() + 0x177E9A0) as *const c_void,
            first: core::ptr::null(),
        }
    }

    pub fn first(&self) -> Option<&MtProperty> {
        if self.is_empty() {
            None
        } else {
            Some(unsafe { &*self.first })
        }
    }

    pub fn is_empty(&self) -> bool {
        self.first == core::ptr::null()
    }

    pub fn len(&self) -> usize {
        if self.is_empty() {
            return 0;
        }

        let mut len = 0;
        let mut p = self.first;
        while p != core::ptr::null() {
            len += 1;
            p = unsafe { (*p).next };
        }

        len
    }

    pub fn iter(&self) -> MtPropertyListIter<'_> {
        MtPropertyListIter {
            current: self.first,
            _phantom: PhantomData,
        }
    }
}

impl<'a> Iterator for MtPropertyListIter<'a> {
    type Item = &'a MtProperty;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            return None;
        }
        let current = unsafe { &*self.current };
        self.current = current.next;
        Some(current)
    }
}

impl<'a> IntoIterator for &'a MtPropertyList {
    type Item = &'a MtProperty;
    type IntoIter = MtPropertyListIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
