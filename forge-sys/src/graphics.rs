#[cfg(feature = "nvn")]
use crate::nvn::types::NVNtexture;
#[cfg(all(feature = "nvn", feature = "imgui"))]
use crate::nvn::types::{NVNtextureHandle, NVNtextureView};

unsafe extern "C" {
    pub fn forge_graphics_isMenuVisible() -> bool;

    #[cfg(feature = "nvn")]
    pub fn forge_graphics_getSwapChainTextures(out_count: *mut usize) -> *const *const NVNtexture;

    #[cfg(all(feature = "nvn", feature = "imgui"))]
    pub fn forge_graphics_registerTexture(texture: *const NVNtexture, view: *const NVNtextureView) -> NVNtextureHandle;

    #[cfg(all(feature = "nvn", feature = "imgui"))]
    pub fn forge_graphics_unregisterTexture(handle: NVNtextureHandle);
}
