use alloc::{borrow::Cow, vec::Vec};

pub mod dds;

pub use dds::DdsError;

/// A decoded image, ready to be uploaded to a texture.
///
/// The pixel data of every mip level is stored tightly packed in the layout the GPU expects
/// (row by row for uncompressed formats, 4x4 blocks for block-compressed formats).
pub struct Image<'a> {
    width: u32,
    height: u32,
    format: ImageFormat,
    color_space: ColorSpace,
    data: Cow<'a, [u8]>,
    mips: Vec<MipRange>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    /// BC1 / DXT1, RGB with optional 1-bit alpha.
    Bc1,
    /// BC2 / DXT3, RGB with explicit 4-bit alpha.
    Bc2,
    /// BC3 / DXT5, RGBA.
    Bc3,
    /// BC4, single unsigned channel.
    Bc4,
    /// BC4, single signed channel.
    Bc4Snorm,
    /// BC5, two unsigned channels.
    Bc5,
    /// BC5, two signed channels.
    Bc5Snorm,
    /// BC7, high quality RGBA.
    Bc7,
    /// 8 bits per channel, RGBA byte order.
    Rgba8,
    /// 8 bits per channel, RGB byte order, 4th byte unused.
    Rgbx8,
    /// 8 bits per channel, BGRA byte order.
    Bgra8,
    /// 8 bits per channel, BGR byte order, 4th byte unused.
    Bgrx8,
    /// Single 8-bit red channel.
    R8,
    /// Single 8-bit alpha channel.
    A8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSpace {
    /// The file does not say (legacy DDS headers). The caller decides.
    Unknown,
    Linear,
    Srgb,
}

struct MipRange {
    offset: usize,
    length: usize,
}

pub struct Mip<'a> {
    pub width: u32,
    pub height: u32,
    pub data: &'a [u8],
}

impl ImageFormat {
    /// Whether the format stores pixels in 4x4 blocks.
    pub fn is_compressed(self) -> bool {
        self.block_bytes().is_some()
    }

    /// Bytes per 4x4 block for block-compressed formats.
    pub fn block_bytes(self) -> Option<usize> {
        match self {
            Self::Bc1 | Self::Bc4 | Self::Bc4Snorm => Some(8),
            Self::Bc2 | Self::Bc3 | Self::Bc5 | Self::Bc5Snorm | Self::Bc7 => Some(16),
            _ => None,
        }
    }

    /// Bytes per pixel for uncompressed formats.
    pub fn pixel_bytes(self) -> Option<usize> {
        match self {
            Self::Rgba8 | Self::Rgbx8 | Self::Bgra8 | Self::Bgrx8 => Some(4),
            Self::R8 | Self::A8 => Some(1),
            _ => None,
        }
    }

    /// Size in bytes of a tightly packed `width` x `height` surface, or `None` on overflow.
    pub fn surface_size(self, width: u32, height: u32) -> Option<usize> {
        let (w, h) = (width as usize, height as usize);
        match (self.block_bytes(), self.pixel_bytes()) {
            (Some(block), _) => w.div_ceil(4).max(1).checked_mul(h.div_ceil(4).max(1))?.checked_mul(block),
            (_, Some(pixel)) => w.checked_mul(h)?.checked_mul(pixel),
            (None, None) => unreachable!("every format is either block-compressed or uncompressed"),
        }
    }

    /// The matching NVN texture format, plus a swizzle to apply if the channel order differs.
    ///
    /// `srgb` selects the sRGB variant where one exists, and is ignored otherwise.
    #[cfg(feature = "nvn")]
    pub fn to_nvn(self, srgb: bool) -> (crate::nvn::Format, Option<[crate::nvn::TextureSwizzle; 4]>) {
        use crate::nvn::{Format, TextureSwizzle as S};

        let pick = |linear, srgb_format| if srgb { srgb_format } else { linear };
        match self {
            Self::Bc1 => (pick(Format::RGBA_DXT1, Format::RGBA_DXT1_SRGB), None),
            Self::Bc2 => (pick(Format::RGBA_DXT3, Format::RGBA_DXT3_SRGB), None),
            Self::Bc3 => (pick(Format::RGBA_DXT5, Format::RGBA_DXT5_SRGB), None),
            Self::Bc4 => (Format::RGTC1_UNORN, None),
            Self::Bc4Snorm => (Format::RGTC1_SNORM, None),
            Self::Bc5 => (Format::RGTC2_UNORM, None),
            Self::Bc5Snorm => (Format::RGTC2_SNORM, None),
            Self::Bc7 => (pick(Format::BPTC_UNORM, Format::BPTC_UNORM_SRGB), None),
            Self::Rgba8 => (pick(Format::RGBA8, Format::RGBA8_SRGB), None),
            Self::Rgbx8 => (pick(Format::RGBX8, Format::RGBX8_SRGB), None),
            Self::Bgra8 => (pick(Format::RGBA8, Format::RGBA8_SRGB), Some([S::B, S::G, S::R, S::A])),
            Self::Bgrx8 => (pick(Format::RGBX8, Format::RGBX8_SRGB), Some([S::B, S::G, S::R, S::One])),
            Self::R8 => (Format::R8, None),
            Self::A8 => (Format::R8, Some([S::Zero, S::Zero, S::Zero, S::R])),
        }
    }
}

impl<'a> Image<'a> {
    /// Parses a DDS file. The image borrows its pixel data from `bytes`, nothing is copied.
    pub fn from_dds(bytes: &'a [u8]) -> Result<Self, DdsError> {
        dds::parse(bytes)
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn format(&self) -> ImageFormat {
        self.format
    }

    pub fn color_space(&self) -> ColorSpace {
        self.color_space
    }

    pub fn mip_count(&self) -> usize {
        self.mips.len()
    }

    /// Returns mip level `level`, where level 0 is the full-size image.
    pub fn mip(&self, level: usize) -> Option<Mip<'_>> {
        let range = self.mips.get(level)?;
        Some(Mip {
            width: (self.width >> level).max(1),
            height: (self.height >> level).max(1),
            data: &self.data[range.offset..range.offset + range.length],
        })
    }

    /// Iterates over all mip levels, starting with the full-size image.
    pub fn mips(&self) -> impl Iterator<Item = Mip<'_>> {
        (0..self.mip_count()).filter_map(|level| self.mip(level))
    }

    /// Converts into an image that owns its pixel data, so it no longer borrows the source bytes.
    pub fn into_owned(self) -> Image<'static> {
        Image {
            width: self.width,
            height: self.height,
            format: self.format,
            color_space: self.color_space,
            data: Cow::Owned(self.data.into_owned()),
            mips: self.mips,
        }
    }
}
