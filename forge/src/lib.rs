#![no_std]

#[cfg(feature = "allocator")]
extern crate alloc;

pub extern crate forge_rt as rt;
pub extern crate forge_sys as sys;

pub use forge_macros::entry;

#[cfg(feature = "allocator")]
mod allocator;

