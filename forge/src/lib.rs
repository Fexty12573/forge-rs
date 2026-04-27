#![no_std]

#[cfg(feature = "allocator")]
extern crate alloc;

pub extern crate forge_rt as rt;
pub extern crate forge_sys as sys;

pub use forge_macros::entry;
pub use forge_macros::hook;

#[cfg(feature = "allocator")]
mod allocator;

pub mod hook;
pub mod log;
pub mod mem;
pub mod patch;

#[cfg(feature = "patterns")]
pub mod pattern;
