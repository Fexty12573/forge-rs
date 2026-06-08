use crate::os::time::TimeSpanType;

#[repr(C)]
pub struct EventType {
    _reserved: [u8; 36],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventClearMode {
    Manual = 0,
    Auto = 1,
}

unsafe extern "C" {
    pub fn nnosInitializeEvent(event: *mut EventType, initially_signaled: bool, clear_mode: EventClearMode);
    pub fn nnosFinalizeEvent(event: *mut EventType);
    pub fn nnosSignalEvent(event: *mut EventType);
    pub fn nnosWaitEvent(event: *mut EventType);
    pub fn nnosTryWaitEvent(event: *mut EventType) -> bool;
    pub fn nnosTimedWaitEvent(event: *mut EventType, timeout: TimeSpanType) -> bool;
    pub fn nnosClearEvent(event: *mut EventType);
}

impl Default for EventType {
    fn default() -> Self {
        Self { _reserved: [0u8; 36] }
    }
}
