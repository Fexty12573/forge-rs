#[repr(C)]
pub struct BarrierType {
    _reserved: [u8; 20],
}

unsafe extern "C" {
    #[link_name = "__nnmusl_pthread_barrier_init"]
    pub fn nnosInitializeBarrier(barrier: *mut BarrierType, num_threads: i32) -> u32;

    #[link_name = "__nnmusl_pthread_barrier_wait"]
    pub fn nnosAwaitBarrier(barrier: *mut BarrierType);

    #[link_name = "__nnmusl_pthread_barrier_destroy"]
    pub fn nnosFinalizeBarrier(barrier: *mut BarrierType);
}
