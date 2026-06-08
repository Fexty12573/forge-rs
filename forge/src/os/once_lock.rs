use core::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    sync::atomic::{AtomicU8, Ordering},
};

use sys::os::EventClearMode;

use crate::os::sync::LightEvent;

const UNINIT: u8 = 0;
const INITING: u8 = 1;
const INIT: u8 = 2;

pub struct OnceLock<T> {
    state: AtomicU8,
    ready: UnsafeCell<LightEvent>,
    value: UnsafeCell<MaybeUninit<T>>,
}

unsafe impl<T: Send + Sync> Sync for OnceLock<T> {}
unsafe impl<T: Send> Send for OnceLock<T> {}

impl<T> OnceLock<T> {
    pub const fn new() -> Self {
        Self {
            state: AtomicU8::new(UNINIT),
            ready: UnsafeCell::new(LightEvent::new(false, EventClearMode::Manual)),
            value: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }

    pub fn get(&self) -> Option<&T> {
        if self.state.load(Ordering::Acquire) == INIT {
            Some(unsafe { (*self.value.get()).assume_init_ref() })
        } else {
            None
        }
    }

    pub fn get_or_init(&self, f: impl FnOnce() -> T) -> &T {
        if self.state.load(Ordering::Acquire) == INIT {
            return unsafe { (*self.value.get()).assume_init_ref() };
        }

        match self
            .state
            .compare_exchange(UNINIT, INITING, Ordering::Acquire, Ordering::Acquire)
        {
            Ok(_) => {
                unsafe { (*self.value.get()).write(f()) };
                self.state.store(INIT, Ordering::Release);
                unsafe { (*self.ready.get()).signal() };
            }
            Err(_) => {
                unsafe { (*self.ready.get()).wait() };
            }
        }

        unsafe { (*self.value.get()).assume_init_ref() }
    }
}

impl<T> Drop for OnceLock<T> {
    fn drop(&mut self) {
        if *self.state.get_mut() == INIT {
            unsafe { (*self.value.get()).assume_init_drop() };
        }
    }
}
