use core::fmt::Write;

use macros::{CacheDti, Object, pure_virtual};

use crate::mt::{
    dti::CacheDti,
    error::{MtError, MtResult},
};

/// MT Framework's file stream class (`MtFile`).
///
/// Wraps a game-side file object, exposing its I/O operations — open, read,
/// write, seek and the various size/position queries — through the underlying
/// vtable. Instances are created through the game's own factory (see
/// [`open`](Self::open)), which is why the type derives
/// [`CacheDti`](macros::CacheDti) to locate its [`MtDti`](crate::mt::dti::MtDti).
///
/// `MtFile` also implements [`core::fmt::Write`], so it can be used as the
/// target of `write!` / `writeln!` to append UTF-8 text.
///
/// # Layout
///
/// `#[repr(C)]` (an opaque, vtable-only type via `#[derive(Object)]`), since it
/// is only ever used by reference to call its virtual functions.
#[derive(Object, CacheDti)]
pub struct MtFile;

/// How a file should be opened, passed to [`MtFile::open`].
#[repr(i32)]
pub enum OpenMode {
    /// No/invalid mode.
    Undefined = 0x10,
    /// Open an existing file for reading.
    Read,
    /// Open an existing file for asynchronous reading.
    ReadAsync,
    /// Create or truncate a file for writing.
    Write,
    /// Open a file for writing, appending to the end.
    Append,
    /// Open a file for both reading and writing.
    ReadWrite,
    /// Open a file for reading and writing, appending to the end.
    ReadWriteAppend,
}

/// Reference point for a [`seek`](MtFile::seek) operation.
#[repr(i32)]
pub enum Origin {
    /// Seek relative to the start of the file.
    Begin,
    /// Seek relative to the current position.
    Current,
    /// Seek relative to the end of the file.
    End,
}

impl MtFile {
    /// Opens the file at `path` with the given [`OpenMode`].
    ///
    /// Locates the `MtFile` class, constructs a new instance through its
    /// factory, and opens the file on it. The path is passed to the game as a
    /// raw byte pointer.
    ///
    /// # Errors
    ///
    /// Returns [`MtError::DtiNotFound`] if the `MtFile` class is missing,
    /// [`MtError::FailedToCreateInstance`] if construction fails, or
    /// [`MtError::FailedToOpenFile`] if the open itself fails.
    pub fn open(path: &str, mode: OpenMode) -> MtResult<&mut Self> {
        let dti = Self::dti().ok_or(MtError::DtiNotFound("MtFile"))?;
        let file: &mut MtFile = dti.new().ok_or(MtError::FailedToCreateInstance("MtFile"))?;

        let success = file.open_impl(path.as_ptr(), mode as i32, false);
        if success { Ok(file) } else { Err(MtError::FailedToOpenFile) }
    }

    /// Reads up to `buffer.len()` bytes into `buffer`, returning the number of
    /// bytes actually read.
    pub fn read(&mut self, buffer: &mut [u8]) -> usize {
        self.read_impl(buffer.as_mut_ptr(), buffer.len())
    }

    /// Writes `data` to the file, returning the number of bytes actually
    /// written.
    pub fn write(&mut self, data: &[u8]) -> usize {
        self.write_impl(data.as_ptr(), data.len())
    }

    /// Moves the file position by `offset` bytes relative to `origin`.
    pub fn seek(&mut self, offset: isize, origin: Origin) {
        self.seek_impl(offset, origin as i32)
    }

    /// Raw open virtual function (vtable slot 6); use [`open`](Self::open).
    #[pure_virtual(6)]
    fn open_impl(&mut self, path: *const u8, mode: i32, interrupt: bool) -> bool;

    /// Closes the file (vtable slot 7).
    #[pure_virtual(7)]
    pub fn close(&mut self);

    /// Raw read virtual function (vtable slot 8); use [`read`](Self::read).
    #[pure_virtual(8)]
    fn read_impl(&mut self, buffer: *mut u8, size: usize) -> usize;

    /// Raw asynchronous-read virtual function (vtable slot 9).
    #[pure_virtual(9)]
    fn read_async_impl(&mut self, buffer: *mut u8, size: usize) -> usize;

    /// Raw await-async-read virtual function (vtable slot 10).
    #[pure_virtual(10)]
    fn read_await_impl(&mut self, buffer: *mut u8, size: usize) -> usize;

    /// Returns `true` while an asynchronous read is in progress (vtable slot
    /// 11).
    #[pure_virtual(11)]
    pub fn is_async_reading(&self) -> bool;

    /// Raw write virtual function (vtable slot 12); use [`write`](Self::write).
    #[pure_virtual(12)]
    fn write_impl(&mut self, buffer: *const u8, size: usize) -> usize;

    /// Raw seek virtual function (vtable slot 13); use [`seek`](Self::seek).
    #[pure_virtual(13)]
    fn seek_impl(&mut self, offset: isize, origin: i32);

    /// Returns the current file position, in bytes (vtable slot 14).
    #[pure_virtual(14)]
    pub fn tell(&self) -> usize;

    /// Returns the file's size, in bytes (vtable slot 15).
    #[pure_virtual(15)]
    pub fn size(&self) -> usize;

    /// Sets the file's size, in bytes (vtable slot 16).
    #[pure_virtual(16)]
    pub fn set_size(&mut self, size: usize);

    /// Returns `true` if the file is open for reading (vtable slot 17).
    #[pure_virtual(17)]
    pub fn is_readable(&self) -> bool;

    /// Returns `true` if the file is open for writing (vtable slot 18).
    #[pure_virtual(18)]
    pub fn is_writable(&self) -> bool;

    /// Returns `true` if the file supports asynchronous reading (vtable slot
    /// 19).
    #[pure_virtual(19)]
    pub fn is_async_readable(&self) -> bool;

    /// Returns the size of the pending asynchronous read, in bytes (vtable slot
    /// 20).
    #[pure_virtual(20)]
    pub fn get_async_read_size(&self) -> usize;
}

impl Write for MtFile {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write(s.as_bytes());
        Ok(())
    }
}
