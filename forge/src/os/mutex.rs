use core::{
    cell::UnsafeCell,
    fmt,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr,
    sync::atomic::{AtomicPtr, Ordering},
};

use sys::os::{
    mutex::*,
    thread::{ThreadType, nnosGetCurrentThread},
};

use crate::os::once_lock::OnceLock;

/// A bare mutual exclusion primitive that does not protect any data by itself.
///
/// Prefer [`Mutex<T>`], which owns the data it protects and hands out access through a guard.
pub struct RawMutex {
    inner: UnsafeCell<MutexType>,
}

impl RawMutex {
    /// Creates a non-recursive mutex.
    pub fn new() -> Self {
        Self::new_internal(false)
    }

    /// Creates a recursive mutex that can be locked multiple times by the same thread.
    pub fn recursive() -> Self {
        Self::new_internal(true)
    }

    /// Destroys the mutex. Called automatically on drop.
    pub fn finalize(&self) {
        unsafe { nnosFinalizeMutex(self.ptr()) };
    }

    /// Acquires the mutex, blocking until it is available.
    pub fn lock(&self) {
        unsafe { nnosLockMutex(self.ptr()) };
    }

    /// Attempts to acquire the mutex without blocking. Returns `true` on success.
    pub fn try_lock(&self) -> bool {
        unsafe { nnosTryLockMutex(self.ptr()) }
    }

    /// Releases the mutex.
    pub fn unlock(&self) {
        unsafe { nnosUnlockMutex(self.ptr()) };
    }

    pub(crate) fn ptr(&self) -> *mut MutexType {
        self.inner.get()
    }

    fn new_internal(recursive: bool) -> Self {
        let mut inner = MutexType::default();
        unsafe { nnosInitializeMutex(&mut inner as *mut MutexType, recursive, 0) };
        Self {
            inner: UnsafeCell::new(inner),
        }
    }
}

impl Drop for RawMutex {
    fn drop(&mut self) {
        self.finalize();
    }
}

/// A mutual exclusion primitive that owns the data it protects.
///
/// Equivalent to `std::sync::Mutex`, except that there is no poisoning (panics do not unwind).
/// [`new`](Self::new) is `const`, so a `Mutex` can be placed directly in a `static`; the underlying
/// OS mutex is created on first use.
///
/// The mutex is not recursive: locking it again on a thread that already holds it panics instead of
/// deadlocking.
///
/// # Example
/// ```ignore
/// struct State {
///     frame: u64,
/// }
///
/// static STATE: Mutex<State> = Mutex::new(State { frame: 0 });
///
/// STATE.lock().frame += 1;
/// STATE.with(|state| state.frame += 1);
/// ```
pub struct Mutex<T: ?Sized> {
    raw: OnceLock<RawMutex>,
    /// The thread currently holding the lock, or null. Only used to detect re-locking on the same thread.
    owner: AtomicPtr<ThreadType>,
    data: UnsafeCell<T>,
}

// SAFETY: the data is only ever accessed by the thread holding the lock, so `T` only has to be `Send`.
unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}
unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    /// Creates a new mutex protecting `value`.
    pub const fn new(value: T) -> Self {
        Self {
            raw: OnceLock::new(),
            owner: AtomicPtr::new(ptr::null_mut()),
            data: UnsafeCell::new(value),
        }
    }

    /// Consumes the mutex and returns the protected value.
    pub fn into_inner(self) -> T {
        self.data.into_inner()
    }
}

impl<T: ?Sized> Mutex<T> {
    /// Acquires the mutex, blocking until it is available.
    ///
    /// # Panics
    /// Panics if the current thread already holds the lock.
    pub fn lock(&self) -> MutexGuard<'_, T> {
        let current = current_thread();
        assert!(!self.is_held_by(current), "Mutex is already locked by the current thread");

        self.raw().lock();
        self.owner.store(current, Ordering::Relaxed);
        MutexGuard::new(self)
    }

    /// Attempts to acquire the mutex without blocking.
    ///
    /// Returns `None` if the mutex is held by any thread, including the current one.
    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        let current = current_thread();
        if self.is_held_by(current) || !self.raw().try_lock() {
            return None;
        }

        self.owner.store(current, Ordering::Relaxed);
        Some(MutexGuard::new(self))
    }

    /// Runs `f` with exclusive access to the protected value.
    ///
    /// # Panics
    /// Panics if the current thread already holds the lock.
    pub fn with<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        f(&mut self.lock())
    }

    /// Returns a mutable reference to the protected value without locking.
    ///
    /// No locking is needed because `&mut self` already guarantees exclusive access.
    pub fn get_mut(&mut self) -> &mut T {
        self.data.get_mut()
    }

    fn raw(&self) -> &RawMutex {
        self.raw.get_or_init(RawMutex::new)
    }

    /// Whether `thread` holds the lock. Only reliable for the current thread: `owner` can only equal the
    /// current thread if the current thread itself stored it while holding the lock.
    fn is_held_by(&self, thread: *mut ThreadType) -> bool {
        self.owner.load(Ordering::Relaxed) == thread
    }
}

impl<T: Default> Default for Mutex<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> From<T> for Mutex<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: ?Sized + fmt::Debug> fmt::Debug for Mutex<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("Mutex");
        match self.try_lock() {
            Some(guard) => d.field("data", &&*guard),
            None => d.field("data", &format_args!("<locked>")),
        };
        d.finish_non_exhaustive()
    }
}

/// Grants access to the data of a locked [`Mutex`]. The mutex is unlocked when the guard is dropped.
#[must_use = "if unused the Mutex will immediately unlock"]
pub struct MutexGuard<'a, T: ?Sized> {
    mutex: &'a Mutex<T>,
    /// The OS mutex must be unlocked by the thread that locked it, so the guard must not be `Send`.
    _not_send: PhantomData<*const ()>,
}

impl<'a, T: ?Sized> MutexGuard<'a, T> {
    fn new(mutex: &'a Mutex<T>) -> Self {
        Self {
            mutex,
            _not_send: PhantomData,
        }
    }

    /// The OS mutex backing this guard, for use with condition variables.
    pub(crate) fn raw(&self) -> &RawMutex {
        self.mutex.raw()
    }

    /// Marks the lock as released by the current thread, before a condition variable unlocks it.
    pub(crate) fn release_owner(&self) {
        self.mutex.owner.store(ptr::null_mut(), Ordering::Relaxed);
    }

    /// Marks the lock as held by the current thread again, after a condition variable relocked it.
    pub(crate) fn reacquire_owner(&self) {
        self.mutex.owner.store(current_thread(), Ordering::Relaxed);
    }
}

impl<T: ?Sized> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<T: ?Sized> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<T: ?Sized> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        self.release_owner();
        self.mutex.raw().unlock();
    }
}

impl<T: ?Sized + fmt::Debug> fmt::Debug for MutexGuard<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

fn current_thread() -> *mut ThreadType {
    unsafe { nnosGetCurrentThread() }
}
