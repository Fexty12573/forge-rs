use core::ffi::c_char;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ForgeVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

#[repr(C)]
pub struct PluginInitParams {
    pub required_version: ForgeVersion,
    /// NUL-terminated display name, shown in the loader's plugin list. Zeroed by the loader.
    pub name: [c_char; 64],
}
