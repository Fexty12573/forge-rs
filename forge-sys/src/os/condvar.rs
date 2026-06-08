use crate::os::{mutex::MutexType, time::TimeSpanType};

#[repr(C)]
#[derive(Default)]
pub struct ConditionVariableType {
    _reserved: [u8; 20], // TODO: Figure out actual size
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ConditionVariableStatus {
    NoTimeout = 0,
    Timeout = 1,
}

unsafe extern "C" {
    pub fn nnosInitializeConditionVariable(var: *mut ConditionVariableType);
    pub fn nnosFinalizeConditionVariable(var: *mut ConditionVariableType);
    pub fn nnosSignalConditionVariable(var: *mut ConditionVariableType);
    pub fn nnosBroadcastConditionVariable(var: *mut ConditionVariableType);
    pub fn nnosWaitConditionVariable(var: *mut ConditionVariableType);
    pub fn nnosTimedWaitConditionVariable(
        var: *mut ConditionVariableType,
        mutex: *mut MutexType,
        timout: TimeSpanType,
    ) -> ConditionVariableStatus;
}
