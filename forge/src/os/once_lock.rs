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

/// A cell that can be written exactly once and is safe to share across threads.
///
/// Equivalent to `std::sync::OnceLock`. The first call to [`get_or_init`](Self::get_or_init)
/// runs the initializer; all subsequent calls block until initialization completes
/// and then return a reference to the stored value.
pub struct OnceLock<T> {
    state: AtomicU8,
    ready: LightEvent,
    value: UnsafeCell<MaybeUninit<T>>,
}

unsafe impl<T: Send + Sync> Sync for OnceLock<T> {}
unsafe impl<T: Send> Send for OnceLock<T> {}

impl<T> OnceLock<T> {
    /// Creates a new uninitialized `OnceLock`.
    pub const fn new() -> Self {
        Self {
            state: AtomicU8::new(UNINIT),
            ready: LightEvent::new(false, EventClearMode::Manual),
            value: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }

    /// Returns a reference to the value if initialized, otherwise `None`.
    pub fn get(&self) -> Option<&T> {
        if self.state.load(Ordering::Acquire) == INIT {
            Some(unsafe { (*self.value.get()).assume_init_ref() })
        } else {
            None
        }
    }

    /// Returns a mutable reference to the value if initialized, otherwise `None`.
    pub fn get_mut(&mut self) -> Option<&mut T> {
        if self.state.load(Ordering::Acquire) == INIT {
            Some(unsafe { (*self.value.get_mut()).assume_init_mut() })
        } else {
            None
        }
    }

    /// Returns the value, initializing it with `f` if this is the first call.
    ///
    /// If another thread is already initializing the value, this call blocks until
    /// initialization completes.
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
                self.ready.signal();
            }
            Err(_) => {
                self.ready.wait();
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
