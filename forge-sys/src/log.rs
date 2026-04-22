use core::ffi::c_char;

extern "C" {
    pub fn forge_log(format: *const c_char, ...);
}
