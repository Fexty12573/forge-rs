use core::{
    ffi::c_void,
    ops::{Deref, DerefMut},
};

use alloc::{boxed::Box, vec::Vec};
use sys::nvn::types::{NVNcommandBuffer, NVNcommandBufferMemoryEvent, NVNdevice};

use crate::nvn::{
    CommandBufferMemoryEvent, MemoryPoolFlags, Owned, command_buffer::CommandBuffer, device::Device, funcs as ffi,
    memory_pool::ManagedMemoryPool, util::AlignedStorage,
};

const COMMAND_MEMORY_SIZE: usize = 0x20000; // 128K
const CONTROL_MEMORY_SIZE: usize = 0x8000; // 32K

const COMMAND_MEM_CHUNK_SIZE: usize = COMMAND_MEMORY_SIZE / 4;
const CONTROL_MEM_CHUNK_SIZE: usize = CONTROL_MEMORY_SIZE / 4;

/// A command buffer that manages its own memory.
/// In case it runs out of memory it will allocate more automatically
/// and reuse old memory where possible.
pub struct ManagedCommandBuffer {
    buffer: Owned<CommandBuffer>,
    chunks: Box<Chunks>,
    cmd_pool: ManagedMemoryPool,
    cnt_mem: AlignedStorage,
}

struct Chunks {
    device: *const NVNdevice,
    cmd_mem_chunks: Vec<ManagedMemoryPool>,
    cnt_mem_chunks: Vec<AlignedStorage>,
    cmd_idx: usize,
    cnt_idx: usize,
}

impl ManagedCommandBuffer {
    /// Creates a new managed command buffer.
    pub fn new(device: &Device) -> Option<ManagedCommandBuffer> {
        let mut buffer = CommandBuffer::new(device)?;
        let cmd_pool = ManagedMemoryPool::new(
            device,
            COMMAND_MEMORY_SIZE,
            MemoryPoolFlags::CPU_UNCACHED | MemoryPoolFlags::GPU_UNCACHED,
        )?;

        let cnt_mem = AlignedStorage::new(CONTROL_MEMORY_SIZE, 8)?;

        let mut chunks = Box::new(Chunks {
            device: device.as_ptr(),
            cmd_mem_chunks: Vec::new(),
            cnt_mem_chunks: Vec::new(),
            cmd_idx: 0,
            cnt_idx: 0,
        });

        buffer.add_command_memory(&cmd_pool, 0, COMMAND_MEMORY_SIZE);
        buffer.add_control_memory(unsafe { cnt_mem.ptr_mut() }.cast(), CONTROL_MEMORY_SIZE);
        buffer.set_memory_callback(memory_callback);
        buffer.set_memory_callback_data((&raw mut *chunks).cast());

        Some(Self {
            buffer,
            chunks,
            cmd_pool,
            cnt_mem,
        })
    }

    /// Resets the internal memory counters of the command buffer.
    /// Should be called at the start of every frame.
    ///
    /// # Safety
    /// Must never be called while commands are still being submitted.
    pub fn reset_memory(&mut self) {
        self.chunks.cmd_idx = 0;
        self.chunks.cnt_idx = 0;

        self.buffer.add_command_memory(&self.cmd_pool, 0, COMMAND_MEMORY_SIZE);
        self.buffer
            .add_control_memory(unsafe { self.cnt_mem.ptr_mut() }.cast(), CONTROL_MEMORY_SIZE);
    }

    fn allocation_callback(buffer: *mut NVNcommandBuffer, chunks: &mut Chunks, event: CommandBufferMemoryEvent, min_size: usize) {
        match event {
            NVNcommandBufferMemoryEvent::OutOfCommandMemory => {
                if min_size > COMMAND_MEM_CHUNK_SIZE {
                    panic!(
                        "Command Buffer requested command memory of size {min_size} which \
                        exceeds the chunk size of {COMMAND_MEM_CHUNK_SIZE}. \
                        Report this issue to the developer."
                    )
                }

                let device = unsafe { Device::from_ptr(chunks.device) };
                if chunks.cmd_idx >= chunks.cmd_mem_chunks.len() {
                    let chunk = ManagedMemoryPool::new(
                        device,
                        COMMAND_MEM_CHUNK_SIZE,
                        MemoryPoolFlags::CPU_UNCACHED | MemoryPoolFlags::GPU_UNCACHED,
                    )
                    .expect("Failed to allocate command buffer command memory");

                    unsafe {
                        ffi::command_buffer_add_command_memory(buffer, chunk.as_ptr(), 0, COMMAND_MEM_CHUNK_SIZE);
                    }
                    chunks.cmd_mem_chunks.push(chunk);
                } else {
                    let chunk = &chunks.cmd_mem_chunks[chunks.cmd_idx];
                    unsafe {
                        ffi::command_buffer_add_command_memory(buffer, chunk.as_ptr(), 0, COMMAND_MEM_CHUNK_SIZE);
                    }
                }

                chunks.cmd_idx += 1;
            }
            NVNcommandBufferMemoryEvent::OutOfControlMemory => {
                if min_size > CONTROL_MEM_CHUNK_SIZE {
                    panic!(
                        "Command Buffer requested control memory of size {min_size} which \
                        exceeds the chunk size of {CONTROL_MEM_CHUNK_SIZE}. \
                        Report this issue to the developer."
                    )
                }

                if chunks.cnt_idx >= chunks.cnt_mem_chunks.len() {
                    let chunk =
                        AlignedStorage::new(CONTROL_MEM_CHUNK_SIZE, 8).expect("Failed to allocate command buffer control memory");

                    unsafe {
                        ffi::command_buffer_add_control_memory(buffer, chunk.ptr_mut().cast(), chunk.size());
                    }
                    chunks.cnt_mem_chunks.push(chunk);
                } else {
                    let chunk = &chunks.cnt_mem_chunks[chunks.cnt_idx];
                    unsafe {
                        ffi::command_buffer_add_control_memory(buffer, chunk.ptr_mut().cast(), chunk.size());
                    }
                }

                chunks.cnt_idx += 1;
            }
        }
    }
}

impl Deref for ManagedCommandBuffer {
    type Target = CommandBuffer;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl DerefMut for ManagedCommandBuffer {
    fn deref_mut(&mut self) -> &mut CommandBuffer {
        &mut self.buffer
    }
}

unsafe extern "C" fn memory_callback(
    buffer: *mut NVNcommandBuffer,
    event: NVNcommandBufferMemoryEvent,
    min_size: usize,
    user_data: *mut c_void,
) {
    let chunks_ptr = user_data.cast::<Chunks>();
    if let Some(chunks) = unsafe { chunks_ptr.as_mut() } {
        ManagedCommandBuffer::allocation_callback(buffer, chunks, event, min_size);
    }
}
