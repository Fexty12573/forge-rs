use sys::os::barrier::*;

pub struct Barrier {
    inner: BarrierType,
}

impl Barrier {
    pub fn new(threads: u32) -> Self {
        let mut inner = BarrierType::default();
        unsafe { nnosInitializeBarrier(&mut inner as *mut BarrierType, threads as i32) };
        Self { inner }
    }

    pub fn finalize(&mut self) {
        unsafe { nnosFinalizeBarrier(self.ptr()) };
    }

    pub fn wait(&mut self) {
        unsafe { nnosAwaitBarrier(self.ptr()) };
    }

    pub(crate) fn ptr(&mut self) -> *mut BarrierType {
        &mut self.inner as *mut BarrierType
    }
}

impl Drop for Barrier {
    fn drop(&mut self) {
        self.finalize();
    }
}
