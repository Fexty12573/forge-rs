use core::{ffi::c_void, marker::PhantomData};

use sys::os::tls::*;

use alloc::boxed::Box;

pub struct ThreadLocal<T> {
    slot: TlsSlot,
    init: fn() -> T,
    _marker: PhantomData<*mut T>,
}

unsafe impl<T: Send> Send for ThreadLocal<T> {}
unsafe impl<T: Send> Sync for ThreadLocal<T> {}

impl<T> ThreadLocal<T> {
    pub fn new(init: fn() -> T) -> Self {
        let mut slot = TlsSlot::new();
        let result = unsafe { nnosAllocateTlsSlot(&mut slot, Self::destructor) };

        if result != 0 || !slot.valid() {
            panic!("Failed to allocate TLS Slot");
        }

        Self {
            slot,
            init,
            _marker: Default::default(),
        }
    }

    pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        f(self.get_or_init())
    }

    pub fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        f(self.get_or_init())
    }

    fn get_or_init(&self) -> &mut T {
        let ptr = unsafe { nnosGetTlsValue(self.slot) };

        if !ptr.is_null() {
            unsafe { &mut *(ptr as *mut T) }
        } else {
            let val = Box::leak(Box::new((self.init)()));
            unsafe { nnosSetTlsValue(self.slot, val as *mut T as *mut c_void) };
            val
        }
    }

    unsafe extern "C" fn destructor(value: *mut c_void) {
        if !value.is_null() {
            drop(unsafe { Box::from_raw(value as *mut T) })
        }
    }
}

impl<T> Drop for ThreadLocal<T> {
    fn drop(&mut self) {
        unsafe { nnosFreeTlsSlot(self.slot) };
    }
}
