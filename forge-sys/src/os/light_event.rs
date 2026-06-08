use crate::os::event::EventClearMode;

#[repr(C)]
#[derive(Default)]
pub struct LightEventType {
    unk0: u32,
    unk1: bool,
    unk2: bool,
    unk3: u16,
    unk4: u32,
}

impl LightEventType {
    pub const fn new(signaled: bool, clear_mode: EventClearMode) -> LightEventType {
        Self {
            unk0: if signaled { 2 } else { 0 },
            unk1: match clear_mode {
                EventClearMode::Manual => false,
                EventClearMode::Auto => true,
            },
            unk2: true,
            unk3: 0,
            unk4: 0,
        }
    }
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
