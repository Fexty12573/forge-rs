//! Functions for querying and controlling the calling thread.

use core::time::Duration;

use sys::os::{TimeSpanType, thread::*};

/// Returns the logical processor number the calling thread is currently running on.
pub fn processor() -> i32 {
    unsafe { nnosGetCurrentProcessorNumber() }
}

/// Returns the core number the calling thread is currently running on.
pub fn core() -> i32 {
    unsafe { nnosGetCurrentCoreNumber() }
}

/// Suspends the calling thread for at least `duration`.
pub fn sleep_for(duration: Duration) {
    unsafe { nnosSleepThread(TimeSpanType(duration.as_nanos() as u64)) };
}

/// Yields the calling thread's remaining timeslice to the scheduler.
pub fn yield_now() {
    unsafe { nnosYieldThread() };
}
