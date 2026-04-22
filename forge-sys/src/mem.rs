extern "C" {
    pub fn forge_mem_getMainTextAddr() -> u32;
    pub fn forge_mem_getMainRoDataAddr() -> u32;
    pub fn forge_mem_getMainDataAddr() -> u32;
    pub fn forge_mem_getMainBssAddr() -> u32;
    pub fn forge_mem_getMainHeapAddr() -> u32;
}
