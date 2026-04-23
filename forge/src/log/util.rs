use core::ffi::c_char;
use core::fmt::{self, Write};

pub struct FixedCStringWriter<'a> {
    buf: &'a mut [u8],
    len: usize,
}

impl<'a> FixedCStringWriter<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        if !buf.is_empty() {
            buf[0] = 0;
        }

        Self { buf, len: 0 }
    }

    pub fn as_c_str_ptr(&self) -> *const c_char {
        self.buf.as_ptr().cast()
    }
}

impl Write for FixedCStringWriter<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if self.buf.is_empty() {
            return Ok(());
        }

        let max_len = self.buf.len() - 1;
        let available = max_len.saturating_sub(self.len);
        let bytes = s.as_bytes();
        let to_copy = available.min(bytes.len());

        for (dst, src) in self.buf[self.len..self.len + to_copy]
            .iter_mut()
            .zip(bytes.iter().take(to_copy))
        {
            *dst = if *src == 0 { b'?' } else { *src };
        }

        self.len += to_copy;
        self.buf[self.len] = 0;
        Ok(())
    }
}
