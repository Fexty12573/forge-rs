use core::{cell::UnsafeCell, ops::Deref};

use crate::os::sync::OnceLock;

/// A value that initializes itself on first access.
///
/// Equivalent to `std::sync::LazyLock`. The initializer `F` is called at most
/// once, and subsequent dereferences return the stored value directly.
pub struct LazyLock<T, F = fn() -> T> {
    once: OnceLock<T>,
    init: UnsafeCell<Option<F>>,
}

unsafe impl<T: Sync + Send, F: Send> Sync for LazyLock<T, F> {}
unsafe impl<T: Send, F: Send> Send for LazyLock<T, F> {}

impl<T, F: FnOnce() -> T> LazyLock<T, F> {
    /// Creates a new `LazyLock` that will call `f` to initialize the value on first access.
    pub const fn new(f: F) -> Self {
        Self {
            once: OnceLock::new(),
            init: UnsafeCell::new(Some(f)),
        }
    }

    /// Forces initialization and returns a reference to the stored value.
    pub fn force(this: &Self) -> &T {
        this.once.get_or_init(|| {
            let f = unsafe { (*this.init.get()).take() };
            match f {
                Some(f) => f(),
                None => unreachable!("LazyLock instance has previously been poisoned"),
            }
        })
    }

    /// Returns a reference to the value if already initialized, otherwise `None`.
    pub fn get(this: &Self) -> Option<&T> {
        this.once.get()
    }

    /// Returns a mutable reference to the value if already initialized, otherwise `None`.
    pub fn get_mut(this: &mut Self) -> Option<&mut T> {
        this.once.get_mut()
    }
}

impl<T, F: FnOnce() -> T> Deref for LazyLock<T, F> {
    type Target = T;

    fn deref(&self) -> &T {
        LazyLock::force(self)
    }
}
