use core::ffi::c_char;

#[repr(C)]
pub enum Level {
    Debug = 0,
    Info = 1,
    Warn = 2,
    Error = 3,
}

unsafe extern "C" {
    pub fn forge_log(level: Level, format: *const c_char, ...);
    pub fn forge_log_getLevel() -> Level;
}
