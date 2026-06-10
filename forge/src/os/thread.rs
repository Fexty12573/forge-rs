use alloc::{
    alloc::{alloc, dealloc, handle_alloc_error},
    boxed::Box,
};
use core::{alloc::Layout, cell::UnsafeCell, ffi::c_void, ptr::NonNull};

use sys::os::thread::*;

pub const STACK_ALIGNMENT: usize = 0x1000;

pub struct Thread {
    inner: Box<UnsafeCell<ThreadType>>,
    _stack: Stack,
}

struct AlignedStack {
    ptr: NonNull<u8>,
    size: usize,
}

#[allow(dead_code)]
enum Stack {
    Static(&'static mut [u8]),
    Dynamic(AlignedStack),
}

pub fn spawn<F: FnOnce() + Send + 'static>(f: F, stack_size: usize, priority: i32) -> Thread {
    let t = spawn_halted(f, stack_size, priority);
    unsafe { nnosStartThread(t.inner.get()) };
    t
}

pub fn spawn_with<F: FnOnce() + Send + 'static>(f: F, stack: &'static mut [u8], priority: i32) -> Thread {
    let t = spawn_halted_with(f, stack, priority);
    unsafe { nnosStartThread(t.inner.get()) };
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
}

impl Drop for Thread {
    fn drop(&mut self) {
        unsafe {
            nnosWaitThread(self.inner.get());
            nnosDestroyThread(self.inner.get());
        }
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
