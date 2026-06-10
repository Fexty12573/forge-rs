use core::ffi::{c_char, c_void};

use crate::os::TimeSpanType;

#[repr(C)]
#[repr(align(16))]
pub struct ThreadType {
    reserved: [u8; 0x1C0],
}

pub type ThreadFn = unsafe extern "C" fn(arg: *mut c_void);

unsafe extern "C" {
    pub fn nnosCreateThread(
        t: *mut ThreadType,
        entry: ThreadFn,
        arg: *mut c_void,
        stack: *mut c_void,
        stack_size: usize,
        priority: i32,
    ) -> u32;

    pub fn nnosCreateThreadWithCoreNumber(
        t: *mut ThreadType,
        entry: ThreadFn,
        arg: *mut c_void,
        stack: *mut c_void,
        stack_size: usize,
        priority: i32,
        ideal_core: i32,
    ) -> u32;

    pub fn nnosDestroyThread(t: *mut ThreadType);
    pub fn nnosStartThread(t: *mut ThreadType);
    pub fn nnosGetCurrentThread() -> *mut ThreadType;
    pub fn nnosTryWaitThread(t: *mut ThreadType) -> bool;
    pub fn nnosWaitThread(t: *mut ThreadType);
    pub fn nnosYieldThread();
    pub fn nnosSleepThread(time: TimeSpanType);
    pub fn nnosChangeThreadPriority(t: *const ThreadType, priority: i32) -> i32;
    pub fn nnosGetThreadPriority(t: *const ThreadType) -> i32;
    pub fn nnosGetThreadCurrentPriority(t: *const ThreadType) -> i32;
    pub fn nnosSetThreadName(t: *mut ThreadType, name: *const c_char);
    pub fn nnosSetThreadNamePointer(t: *mut ThreadType, name: *const c_char);
    pub fn nnosGetThreadNamePointer(t: *const ThreadType) -> *const c_char;
    pub fn nnosGetCurrentProcessorNumber() -> i32;
    pub fn nnosGetCurrentCoreNumber() -> i32;

    #[link_name = "_ZN2nn2os11GetThreadIdEPKNS0_10ThreadTypeE"]
    pub fn nnosGetThreadId(t: *const ThreadType) -> u64;
}
