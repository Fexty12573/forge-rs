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

pub struct Thread {
    started: UnsafeCell<bool>,
    inner: Box<UnsafeCell<ThreadType>>,
    _stack: Stack,
}

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

pub fn spawn<F: FnOnce() + Send + 'static>(f: F, stack_size: usize, priority: i32) -> Thread {
    let t = spawn_halted(f, stack_size, priority);
    t.start();
    t
}

pub fn spawn_with<F: FnOnce() + Send + 'static>(f: F, stack: &'static mut [u8], priority: i32) -> Thread {
    let t = spawn_halted_with(f, stack, priority);
    t.start();
    t
}

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
    pub fn is_finished(&self) -> bool {
        unsafe { nnosTryWaitThread(self.inner.get()) }
    }

    pub fn join(&self) {
        unsafe { nnosWaitThread(self.inner.get()) };
    }

    pub fn id(&self) -> u64 {
        unsafe { nnosGetThreadId(self.inner.get()) }
    }

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
    pub const DEFAULT_STACK_SIZE: usize = 0x2000;

    pub fn new() -> Self {
        Self {
            name: None,
            halted: false,
            stack: EitherOr::B(Self::DEFAULT_STACK_SIZE),
            priority: 16,
        }
    }

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

    pub fn halted(mut self) -> Self {
        self.halted = true;
        self
    }

    pub fn stack(mut self, stack: &'static mut [u8]) -> Self {
        self.stack = EitherOr::A(stack);
        self
    }

    pub fn stack_size(mut self, size: usize) -> Self {
        self.stack = EitherOr::B(size);
        self
    }

    pub fn priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

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
