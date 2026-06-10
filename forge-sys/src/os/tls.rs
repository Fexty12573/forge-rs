use core::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TlsSlot {
    slot: u32,
}

pub type TlsDestructor = unsafe extern "C" fn(value: *mut c_void);

unsafe extern "C" {
    pub fn nnosAllocateTlsSlot(slot: *mut TlsSlot, destructor: TlsDestructor) -> u32;
    pub fn nnosFreeTlsSlot(slot: TlsSlot);
    pub fn nnosGetTlsValue(slot: TlsSlot) -> *mut c_void;
    pub fn nnosSetTlsValue(slot: TlsSlot, value: *mut c_void);
    pub fn nnosGetUsedTlsSlotCount() -> i32;
}

impl TlsSlot {
    const INVALID: u32 = 0xFFFFFFFF;

    pub fn new() -> Self {
        Self { slot: Self::INVALID }
    }

    pub fn valid(self) -> bool {
        self.slot != Self::INVALID
    }
}
