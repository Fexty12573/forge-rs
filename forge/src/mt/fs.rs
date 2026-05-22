use core::fmt::Write;

use macros::{CacheDti, Object, pure_virtual};

use crate::mt::dti::CacheDti;

#[derive(Object, CacheDti)]
pub struct MtFile;

#[repr(i32)]
pub enum OpenMode {
    Undefined = 0x10,
    Read,
    ReadAsync,
    Write,
    Append,
    ReadWrite,
    ReadWriteAppend,
}

#[repr(i32)]
pub enum Origin {
    Begin,
    Current,
    End,
}

impl MtFile {
    pub fn open(path: &str, mode: OpenMode) -> Option<&mut Self> {
        let file: &mut MtFile = Self::dti()?.new()?;
        let success = file.open_impl(path.as_ptr(), mode as i32, false);
        if success { Some(file) } else { None }
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> usize {
        self.read_impl(buffer.as_mut_ptr(), buffer.len())
    }

    pub fn write(&mut self, data: &[u8]) -> usize {
        self.write_impl(data.as_ptr(), data.len())
    }

    pub fn seek(&mut self, offset: isize, origin: Origin) {
        self.seek_impl(offset, origin as i32)
    }

    #[pure_virtual(6)]
    fn open_impl(&mut self, path: *const u8, mode: i32, interrupt: bool) -> bool;

    #[pure_virtual(7)]
    pub fn close(&mut self);

    #[pure_virtual(8)]
    fn read_impl(&mut self, buffer: *mut u8, size: usize) -> usize;

    #[pure_virtual(9)]
    fn read_async_impl(&mut self, buffer: *mut u8, size: usize) -> usize;

    #[pure_virtual(10)]
    fn read_await_impl(&mut self, buffer: *mut u8, size: usize) -> usize;

    #[pure_virtual(11)]
    pub fn is_async_reading(&self) -> bool;

    #[pure_virtual(12)]
    fn write_impl(&mut self, buffer: *const u8, size: usize) -> usize;

    #[pure_virtual(13)]
    fn seek_impl(&mut self, offset: isize, origin: i32);

    #[pure_virtual(14)]
    pub fn tell(&self) -> usize;

    #[pure_virtual(15)]
    pub fn size(&self) -> usize;

    #[pure_virtual(16)]
    pub fn set_size(&mut self, size: usize);

    #[pure_virtual(17)]
    pub fn is_readable(&self) -> bool;

    #[pure_virtual(18)]
    pub fn is_writable(&self) -> bool;

    #[pure_virtual(19)]
    pub fn is_async_readable(&self) -> bool;

    #[pure_virtual(20)]
    pub fn get_async_read_size(&self) -> usize;
}

impl Write for MtFile {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write(s.as_bytes());
        Ok(())
    }
}
