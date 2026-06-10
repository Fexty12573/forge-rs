use core::time::Duration;

use sys::os::{TimeSpanType, thread::*};

pub fn processor() -> i32 {
    unsafe { nnosGetCurrentProcessorNumber() }
}

pub fn core() -> i32 {
    unsafe { nnosGetCurrentCoreNumber() }
}

pub fn sleep_for(duration: Duration) {
    unsafe { nnosSleepThread(TimeSpanType(duration.as_nanos() as u64)) };
}
