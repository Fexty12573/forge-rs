use core::ops::{Deref, DerefMut};

use crate::nvn::{
    MemoryPoolFlags_, Owned,
    device::Device,
    memory_pool::MemoryPool,
    util::{AlignedStorage, align_up},
};

pub struct ManagedMemoryPool {
    pool: Owned<MemoryPool>,
    _storage: AlignedStorage,
}

impl ManagedMemoryPool {
    /// Creates a new [`ManagedMemoryPool`] with the given size and flags.
    pub fn new(device: &Device, size: usize, flags: MemoryPoolFlags_) -> Option<Self> {
        if size == 0 {
            return None;
        }

        const ALIGNMENT: usize = 0x1000;
        const SIZE_ALIGNMENT: usize = 0x1000;
        let storage = AlignedStorage::new(align_up(size, SIZE_ALIGNMENT), ALIGNMENT)?;

        let pool = unsafe { MemoryPool::with_storage(device, storage.ptr_mut().cast(), storage.size(), flags) }?;
        Some(Self { pool, _storage: storage })
    }
}

impl Deref for ManagedMemoryPool {
    type Target = MemoryPool;

    fn deref(&self) -> &Self::Target {
        &self.pool
    }
}

impl DerefMut for ManagedMemoryPool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pool
    }
}
