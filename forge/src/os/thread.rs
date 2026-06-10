use alloc::{
    alloc::{alloc, dealloc, handle_alloc_error},
    boxed::Box,
};
use core::{
    alloc::Layout,
    cell::UnsafeCell,
    ffi::{c_char, c_void},
    ptr::NonNull,
};

use sys::os::thread::*;

pub const STACK_ALIGNMENT: usize = 0x1000;

/// An OS thread handle.
///
/// The thread is joined and destroyed automatically when the handle is dropped.
pub struct Thread {
    started: UnsafeCell<bool>,
    inner: Box<UnsafeCell<ThreadType>>,
    _stack: Stack,
}

/// A builder for configuring and spawning threads.
///
/// # Examples
/// ```ignore
/// let t = Builder::new()
///     .name("worker")
///     .priority(16)
///     .stack_size(0x4000)
///     .spawn(|| { /* ... */ });
/// ```
pub struct Builder {
    name: Option<[c_char; 32]>,
    halted: bool,
    stack: EitherOr<&'static mut [u8], usize>,
    priority: i32,
}

struct AlignedStack {
    ptr: NonNull<u8>,
    size: usize,
}

enum EitherOr<A, B> {
    A(A),
    B(B),
}

#[allow(dead_code)]
enum Stack {
    Static(&'static mut [u8]),
    Dynamic(AlignedStack),
}

/// Spawns a new thread that runs `f`, allocating a dynamic stack of `stack_size` bytes.
pub fn spawn<F: FnOnce() + Send + 'static>(f: F, stack_size: usize, priority: i32) -> Thread {
    let t = spawn_halted(f, stack_size, priority);
    t.start();
    t
}

/// Spawns a new thread that runs `f`, using the provided static `stack` buffer.
pub fn spawn_with<F: FnOnce() + Send + 'static>(f: F, stack: &'static mut [u8], priority: i32) -> Thread {
    let t = spawn_halted_with(f, stack, priority);
    t.start();
    t
}

/// Like [`spawn`], but the thread is created in a halted state and must be started with [`Thread::start`].
pub fn spawn_halted<F: FnOnce() + Send + 'static>(f: F, stack_size: usize, priority: i32) -> Thread {
    let thread = Box::new(UnsafeCell::new(unsafe { core::mem::zeroed() }));
    let arg = Box::into_raw(Box::new(f)) as *mut c_void;
    let mut stack = AlignedStack::new(stack_size);

    unsafe {
        if nnosCreateThread(
            thread.get(),
            thread_entry::<F>,
            arg,
            stack.as_mut_ptr() as *mut c_void,
            stack.len(),
            priority,
        ) != 0
        {
            drop(Box::<F>::from_raw(arg as *mut F));
            panic!("Failed to create thread");
        }
    }

    Thread {
        started: UnsafeCell::new(false),
        inner: thread,
        _stack: Stack::Dynamic(stack),
    }
}

/// Like [`spawn_with`], but the thread is created in a halted state and must be started with [`Thread::start`].
pub fn spawn_halted_with<F: FnOnce() + Send + 'static>(f: F, stack: &'static mut [u8], priority: i32) -> Thread {
    let thread = Box::new(UnsafeCell::new(unsafe { core::mem::zeroed() }));
    let arg = Box::into_raw(Box::new(f)) as *mut c_void;

    unsafe {
        if nnosCreateThread(
            thread.get(),
            thread_entry::<F>,
            arg,
            stack.as_mut_ptr() as *mut c_void,
            stack.len(),
            priority,
        ) != 0
        {
            drop(Box::<F>::from_raw(arg as *mut F));
            panic!("Failed to create thread");
        }
    }

    Thread {
        started: UnsafeCell::new(false),
        inner: thread,
        _stack: Stack::Static(stack),
    }
}

unsafe extern "C" fn thread_entry<F: FnOnce()>(arg: *mut c_void) {
    let f = unsafe { Box::from_raw(arg as *mut F) };
    f();
}

impl Thread {
    /// Returns `true` if the thread has finished executing.
    pub fn is_finished(&self) -> bool {
        unsafe { nnosTryWaitThread(self.inner.get()) }
    }

    /// Blocks until the thread finishes executing.
    pub fn join(&self) {
        unsafe { nnosWaitThread(self.inner.get()) };
    }

    /// Returns the OS thread ID.
    pub fn id(&self) -> u64 {
        unsafe { nnosGetThreadId(self.inner.get()) }
    }

    /// Sets the thread's name, truncated to 31 bytes.
    pub fn set_name(&self, name: &str) {
        let mut buffer = [0 as c_char; 32];
        let bytes = name.as_bytes();
        let len = bytes.len().min(31);
        for i in 0..len {
            buffer[i] = bytes[i] as c_char;
        }

        unsafe {
            nnosSetThreadName(self.inner.get(), buffer.as_ptr());
        }
    }

    /// Starts the thread. Only meaningful for threads created with [`spawn_halted`] or [`Builder::halted`].
    pub fn start(&self) {
        unsafe {
            nnosStartThread(self.inner.get());
            *self.started.get() = true;
        }
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
        unsafe {
            nnosWaitThread(self.inner.get());
            nnosDestroyThread(self.inner.get());
        }
    }
}

impl Builder {
    /// Default stack size used when none is specified (8 KiB).
    pub const DEFAULT_STACK_SIZE: usize = 0x2000;

    /// Creates a builder with default settings: 8 KiB dynamic stack, priority 16, started immediately.
    pub fn new() -> Self {
        Self {
            name: None,
            halted: false,
            stack: EitherOr::B(Self::DEFAULT_STACK_SIZE),
            priority: 16,
        }
    }

    /// Sets the thread name, truncated to 31 bytes.
    pub fn name(mut self, name: &str) -> Self {
        let mut buffer = [0 as c_char; 32];
        let bytes = name.as_bytes();
        let len = bytes.len().min(31);
        for i in 0..len {
            buffer[i] = bytes[i] as c_char;
        }
        self.name = Some(buffer);
        self
    }

    /// Creates the thread in a halted state; call [`Thread::start`] to begin execution.
    pub fn halted(mut self) -> Self {
        self.halted = true;
        self
    }

    /// Uses a caller-provided static buffer as the thread stack instead of allocating one.
    pub fn stack(mut self, stack: &'static mut [u8]) -> Self {
        self.stack = EitherOr::A(stack);
        self
    }

    /// Sets the size in bytes for the dynamically allocated stack.
    pub fn stack_size(mut self, size: usize) -> Self {
        self.stack = EitherOr::B(size);
        self
    }

    /// Sets the thread scheduling priority.
    pub fn priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    /// Consumes the builder and spawns a thread running `f`.
    pub fn spawn<F: FnOnce() + Send + 'static>(self, f: F) -> Thread {
        let thread = match self.stack {
            EitherOr::A(stack) => spawn_halted_with(f, stack, self.priority),
            EitherOr::B(size) => spawn_halted(f, size, self.priority),
        };

        if let Some(name) = self.name {
            unsafe {
                nnosSetThreadName(thread.inner.get(), name.as_ptr());
            }
        }

        if !self.halted {
            thread.start();
        }

        thread
    }
}

impl AlignedStack {
    fn new(size: usize) -> Self {
        let layout = Layout::from_size_align(size, STACK_ALIGNMENT).expect("Invalid Stack Layout");
        let ptr = unsafe { alloc(layout) };
        NonNull::new(ptr)
            .map(|ptr| Self { ptr, size })
            .unwrap_or_else(|| handle_alloc_error(layout))
    }

    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.ptr.as_ptr()
    }

    fn len(&self) -> usize {
        self.size
    }
}

impl Drop for AlignedStack {
    fn drop(&mut self) {
        unsafe {
            dealloc(
                self.as_mut_ptr(),
                Layout::from_size_align_unchecked(self.size, STACK_ALIGNMENT),
            );
        }
    }
}
