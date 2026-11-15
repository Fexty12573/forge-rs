use core::fmt;

use alloc::{borrow::Cow, vec, vec::Vec};

use super::{ColorSpace, Image, ImageFormat, MipRange};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PngError {
    /// The data does not start with the PNG signature.
    NotPng,
    /// The image is interlaced (Adam7), which is not supported. Re-export it without interlacing.
    Interlaced,
    /// The decoded image would not fit in memory.
    TooLarge,
    /// The file is truncated or corrupt.
    Corrupt,
}

impl fmt::Display for PngError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotPng => write!(f, "data is not a PNG file"),
            Self::Interlaced => write!(f, "interlaced PNGs are not supported"),
            Self::TooLarge => write!(f, "PNG is too large to decode"),
            Self::Corrupt => write!(f, "PNG file is truncated or corrupt"),
        }
    }
}

impl From<minipng::Error> for PngError {
    fn from(err: minipng::Error) -> Self {
        match err {
            minipng::Error::NotPng => Self::NotPng,
            minipng::Error::UnsupportedInterlace => Self::Interlaced,
            minipng::Error::TooLargeForUsize => Self::TooLarge,
            _ => Self::Corrupt,
        }
    }
}

/// Decodes a PNG file to 8-bit RGBA.
pub fn decode(bytes: &[u8]) -> Result<Image<'static>, PngError> {
    let header = minipng::decode_png_header(bytes)?;
    let mut buf = vec![0u8; header.required_bytes_rgba8bpc()];

    let mut image = minipng::decode_png(bytes, &mut buf)?;
    image.convert_to_rgba8bpc()?;

    let (width, height) = (image.width(), image.height());
    // `pixels()` always starts at the beginning of `buf`, so the decoded RGBA data is its prefix
    let length = image.pixels().len();
    buf.truncate(length);

    Ok(Image {
        width,
        height,
        format: ImageFormat::Rgba8,
        color_space: ColorSpace::Srgb,
        data: Cow::Owned(buf),
        mips: Vec::from([MipRange { offset: 0, length }]),
    })
}
