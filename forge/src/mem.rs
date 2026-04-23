use sys::mem::*;

/// Returns the base address of the game's text segment.
pub fn text_addr() -> u32 {
    unsafe { forge_mem_getMainTextAddr() }
}

/// Returns the base address of the game's read-only data segment.
pub fn rodata_addr() -> u32 {
    unsafe { forge_mem_getMainRoDataAddr() }
}

/// Returns the base address of the game's data segment.
pub fn data_addr() -> u32 {
    unsafe { forge_mem_getMainDataAddr() }
}

/// Returns the base address of the game's BSS segment.
pub fn bss_addr() -> u32 {
    unsafe { forge_mem_getMainBssAddr() }
}

/// Returns the base address of the game's heap segment.
pub fn heap_addr() -> u32 {
    unsafe { forge_mem_getMainHeapAddr() }
}
