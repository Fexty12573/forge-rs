use core::alloc::{GlobalAlloc, Layout};

use forge_sys::mem::{free, malloc, realloc};

struct ForgeAllocator;

#[global_allocator]
static GLOBAL_ALLOCATOR: ForgeAllocator = ForgeAllocator;

#[repr(C)]
struct Header {
    base: *mut u8,
}

impl ForgeAllocator {
    const HEADER_SIZE: usize = core::mem::size_of::<Header>();

    fn is_header_compatible(layout: Layout) -> bool {
        layout.align() <= core::mem::align_of::<Header>()
    }

    unsafe fn alloc_with_header(layout: Layout) -> *mut u8 {
        let align = layout.align();
        let size = layout.size();
        let total = match size.checked_add(align).and_then(|n| n.checked_add(Self::HEADER_SIZE)) {
            Some(n) => n,
            None => return core::ptr::null_mut(),
        };

        let base = unsafe { malloc(total) as *mut u8 };
        if base.is_null() {
            return core::ptr::null_mut();
        }

        let start = unsafe { base.add(Self::HEADER_SIZE) } as usize;
        let aligned = (start + (align - 1)) & !(align - 1);
        let user = aligned as *mut u8;

        let header = unsafe { (user as *mut Header).sub(1) };
        unsafe {
            *header = Header { base };
        }

        user
    }
}

unsafe impl GlobalAlloc for ForgeAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.size() == 0 {
            return layout.align() as *mut u8;
        }

        if Self::is_header_compatible(layout) {
            unsafe { malloc(layout.size()) as *mut u8 }
        } else {
            unsafe { Self::alloc_with_header(layout) }
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if layout.size() == 0 {
            return;
        }

        if Self::is_header_compatible(layout) {
            unsafe {
                free(ptr.cast());
            }
            return;
        }

        let header = unsafe { (ptr as *mut Header).sub(1) };
        unsafe {
            free((*header).base.cast());
        }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        if layout.size() == 0 {
            return unsafe { self.alloc(Layout::from_size_align_unchecked(new_size, layout.align())) };
        }

        if new_size == 0 {
            unsafe {
                self.dealloc(ptr, layout);
            }
            return layout.align() as *mut u8;
        }

        if Self::is_header_compatible(layout) {
            unsafe { realloc(ptr.cast(), new_size) as *mut u8 }
        } else {
            let new_layout = unsafe { Layout::from_size_align_unchecked(new_size, layout.align()) };
            let new_ptr = unsafe { self.alloc(new_layout) };
            if new_ptr.is_null() {
                return core::ptr::null_mut();
            }

            let to_copy = core::cmp::min(layout.size(), new_size);
            unsafe {
                core::ptr::copy_nonoverlapping(ptr, new_ptr, to_copy);
                self.dealloc(ptr, layout);
            }
            new_ptr
        }
    }
}
