use core::ffi::c_char;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Pattern {
    pub pattern: *const PatternByte,
    pub length: usize,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PatternByte {
    pub value: u8,
    pub mask: u8,
}

unsafe extern "C" {
    pub fn forge_pattern_create(pattern: *const c_char) -> Pattern;
    pub fn forge_pattern_createBits(pattern: *const c_char) -> Pattern;
    pub fn forge_pattern_destroy(pattern: Pattern);

    pub fn forge_pattern_find(pattern: *const c_char) -> u32;
    pub fn forge_pattern_findFrom(start_addr: u32, pattern: *const c_char) -> u32;

    pub fn forge_pattern_findBits(bits: *const c_char) -> u32;
    pub fn forge_pattern_findBitsFrom(start_addr: u32, bits: *const c_char) -> u32;

    pub fn forge_pattern_findEx(pattern: Pattern) -> u32;
    pub fn forge_pattern_findFromEx(start_addr: u32, pattern: Pattern) -> u32;
}
