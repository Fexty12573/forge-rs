use alloc::{ffi::CString, vec::Vec};
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

/// A compiled pattern that can be searched for multiple times efficiently.
pub struct Pattern {
    inner: SysPattern,
}

impl Pattern {
    /// Creates a new pattern from a string (e.g. "48 8B ?? ?? ?? 00").
    pub fn from_str(pat: &str) -> Option<Self> {
        let cstr = CString::new(pat).ok()?;
        let pattern = unsafe { forge_pattern_create(cstr.as_ptr()) };
        if pattern.pattern.is_null() || pattern.length == 0 {
            None
        } else {
            Some(Self { inner: pattern })
        }
    }

    /// Creates a new pattern from a string of bits (e.g. "1101..00").
    pub fn from_bits(pat: &str) -> Option<Self> {
        let cstr = CString::new(pat).ok()?;
        let pattern = unsafe { forge_pattern_createBits(cstr.as_ptr()) };
        if pattern.pattern.is_null() || pattern.length == 0 {
            None
        } else {
            Some(Self { inner: pattern })
        }
    }

    /// Returns the length of this pattern in bytes.
    pub fn len(&self) -> usize {
        self.inner.length
    }

    /// Returns a slice of the pattern bytes, where each byte is either a specific value or a wildcard.
    pub fn bytes(&self) -> &[PatternByte] {
        unsafe { core::slice::from_raw_parts(self.inner.pattern, self.inner.length) }
    }

    /// Returns `true` if this pattern is valid and can be searched for.
    pub fn valid(&self) -> bool {
        !self.inner.pattern.is_null() && self.inner.length != 0
    }

    /// Searches for this pattern in memory, returning the address of the first match if found.
    pub fn find(&self) -> Option<u32> {
        let addr = unsafe { forge_pattern_findEx(self.inner) };
        if addr == 0 { None } else { Some(addr) }
    }

    /// Searches for this pattern in memory starting from a specific address, returning the address of the first match if found.
    pub fn find_from(&self, start_addr: u32) -> Option<u32> {
        let addr = unsafe { forge_pattern_findFromEx(start_addr, self.inner) };
        if addr == 0 { None } else { Some(addr) }
    }

    /// Searches for all occurrences of this pattern in memory, up to a specified maximum number of results.
    pub fn find_all(&self, max_results: usize) -> Vec<u32> {
        let mut results = Vec::with_capacity(max_results);
        let mut count = 0;
        let mut result = self.find();
        while let Some(addr) = result {
            results.push(addr);
            count += 1;
            if count >= max_results {
                break;
            }
            result = self.find_from(addr + 1);
        }

        results
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
