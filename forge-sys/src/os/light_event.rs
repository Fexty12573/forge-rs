use crate::os::event::EventClearMode;

#[repr(C)]
pub struct LightEventType {
    _reserved: [u8; 12],
}

unsafe extern "C" {
    pub fn forge_nnosInitializeLightEvent(event: *mut LightEventType, initially_signaled: bool, clear_mode: EventClearMode);
    pub fn forge_nnosFinalizeLightEvent(event: *mut LightEventType);
    pub fn forge_nnosSignalLightEvent(event: *mut LightEventType);
    pub fn forge_nnosWaitLightEvent(event: *mut LightEventType);
    pub fn forge_nnosTryWaitLightEvent(event: *mut LightEventType) -> bool;
    pub fn forge_nnosTimedWaitLightEvent(event: *mut LightEventType, timeout: crate::os::time::TimeSpanType) -> bool;
    pub fn forge_nnosClearLightEvent(event: *mut LightEventType);
}
