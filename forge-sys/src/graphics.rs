#[cfg(feature = "nvn")]
use crate::nvn::types::NVNtexture;

unsafe extern "C" {
    pub fn forge_graphics_isMenuVisible() -> bool;

    #[cfg(feature = "nvn")]
    pub fn forge_graphics_getSwapChainTextures(out_count: *mut usize) -> *const *const NVNtexture;
}
