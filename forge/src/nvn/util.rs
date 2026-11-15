use alloc::alloc;
use core::{alloc::Layout, ptr::NonNull};

pub fn align_up(value: usize, align: usize) -> usize {
    (value + align - 1) & !(align - 1)
}

pub struct AlignedStorage {
    ptr: NonNull<u8>,
    layout: Layout,
}

// SAFETY: `AlignedStorage` uniquely owns its allocation, like a `Box<[u8]>`. The pointer is never
// shared with another owner, and `&self` only hands out raw pointers, whose use is already `unsafe`.
unsafe impl Send for AlignedStorage {}
unsafe impl Sync for AlignedStorage {}

impl AlignedStorage {
    /// Allocates a block of memory with the given size and alignment.
    pub fn new(size: usize, align: usize) -> Option<Self> {
        if size == 0 {
            return None;
        }

        let layout = Layout::from_size_align(size, align).ok()?;
        let ptr = NonNull::new(unsafe { alloc::alloc_zeroed(layout) })?;

        Some(Self { ptr, layout })
    }

    pub unsafe fn ptr(&self) -> *const u8 {
        self.ptr.as_ptr()
    }

    pub unsafe fn ptr_mut(&self) -> *mut u8 {
        self.ptr.as_ptr()
    }

    pub fn size(&self) -> usize {
        self.layout.size()
    }

    pub fn align(&self) -> usize {
        self.layout.align()
    }
}

impl Drop for AlignedStorage {
    fn drop(&mut self) {
        unsafe {
            alloc::dealloc(self.ptr.as_ptr(), self.layout);
        }
    }
}
