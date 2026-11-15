use alloc::boxed::Box;
use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;

pub(crate) mod sealed {
    pub trait Sealed: Sized {
        /// Unregistered storage for the object, to be initialized in place.
        fn uninit() -> Self;

        /// # Safety
        /// The object must have been successfully initialized and not finalized yet.
        unsafe fn finalize(&mut self);
    }
}

/// An NVN object that is registered with the driver by its address.
///
/// These objects must never move after they have been initialized, which is why they only exist behind an
/// [`Owned`] (or a reference handed out by the game, see [`Device`](super::device::Device) and
/// [`Queue`](super::queue::Queue)).
pub trait Object: sealed::Sealed {}

/// A heap-allocated, initialized NVN object. Finalizes the object when dropped.
///
/// The object stays at the same address for its entire lifetime, so the handle itself can be moved freely.
/// Do not `mem::swap`/`mem::replace` the object behind it, as that moves the registered object.
pub struct Owned<T: Object>(NonNull<T>);

impl<T: Object> Owned<T> {
    /// Allocates storage for `T` and initializes it in place with `init`.
    ///
    /// Returns `None` if `init` reports failure, in which case the object is not finalized.
    pub(crate) fn try_new(init: impl FnOnce(&mut T) -> bool) -> Option<Self> {
        let ptr = NonNull::from(Box::leak(Box::new(T::uninit())));
        if init(unsafe { &mut *ptr.as_ptr() }) {
            Some(Self(ptr))
        } else {
            unsafe { drop(Box::from_raw(ptr.as_ptr())) };
            None
        }
    }
}

impl<T: Object> Deref for Owned<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.0.as_ref() }
    }
}

impl<T: Object> DerefMut for Owned<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.0.as_mut() }
    }
}

impl<T: Object> Drop for Owned<T> {
    fn drop(&mut self) {
        unsafe {
            sealed::Sealed::finalize(self.0.as_mut());
            drop(Box::from_raw(self.0.as_ptr()));
        }
    }
}

// SAFETY: NVN objects may be used from any thread, just not concurrently, which `T: !Sync` already prevents
unsafe impl<T: Object + Send> Send for Owned<T> {}

/// Implements [`Object`] for a `#[repr(transparent)]` wrapper around `UnsafeCell<$raw>`.
macro_rules! impl_object {
    ($ty:ty, $raw:ty, $finalize:path) => {
        impl $crate::nvn::owned::sealed::Sealed for $ty {
            fn uninit() -> Self {
                Self(::core::cell::UnsafeCell::new(<$raw>::default()))
            }

            unsafe fn finalize(&mut self) {
                unsafe { $finalize(self.as_ptr()) }
            }
        }

        impl $crate::nvn::owned::Object for $ty {}
    };
}

pub(crate) use impl_object;
