use alloc::ffi::CString;
use sys::pattern::{Pattern as SysPattern, *};

pub fn find(pattern: &str) -> Option<u32> {
    let cstr = CString::new(pattern).ok()?;
    let addr = unsafe { forge_pattern_find(cstr.as_ptr()) };
    if addr == 0 { None } else { Some(addr) }
}

pub fn find_from(start_addr: u32, pattern: &str) -> Option<u32> {
    let cstr = CString::new(pattern).ok()?;
    let addr = unsafe { forge_pattern_findFrom(start_addr, cstr.as_ptr()) };
    if addr == 0 { None } else { Some(addr) }
}

pub struct Pattern {
    inner: SysPattern,
}

impl Pattern {
    pub fn from_str(pat: &str) -> Option<Self> {
        let cstr = CString::new(pat).ok()?;
        let pattern = unsafe { forge_pattern_create(cstr.as_ptr()) };
        if pattern.pattern.is_null() || pattern.length == 0 {
            None
        } else {
            Some(Self { inner: pattern })
        }
    }

    pub fn from_bits(pat: &str) -> Option<Self> {
        let cstr = CString::new(pat).ok()?;
        let pattern = unsafe { forge_pattern_createBits(cstr.as_ptr()) };
        if pattern.pattern.is_null() || pattern.length == 0 {
            None
        } else {
            Some(Self { inner: pattern })
        }
    }

    pub fn len(&self) -> usize {
        self.inner.length
    }

    pub fn bytes(&self) -> &[PatternByte] {
        unsafe { core::slice::from_raw_parts(self.inner.pattern, self.inner.length) }
    }

    pub fn valid(&self) -> bool {
        !self.inner.pattern.is_null() && self.inner.length != 0
    }

    pub fn find(&self) -> Option<u32> {
        let addr = unsafe { forge_pattern_findEx(self.inner) };
        if addr == 0 { None } else { Some(addr) }
    }

    pub fn find_from(&self, start_addr: u32) -> Option<u32> {
        let addr = unsafe { forge_pattern_findFromEx(start_addr, self.inner) };
        if addr == 0 { None } else { Some(addr) }
    }
}

impl core::ops::Drop for Pattern {
    fn drop(&mut self) {
        unsafe {
            forge_pattern_destroy(self.inner);
            self.inner.pattern = core::ptr::null();
            self.inner.length = 0;
        }
    }
}
