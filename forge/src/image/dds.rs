use core::fmt;

use alloc::{borrow::Cow, vec::Vec};

use super::{ColorSpace, Image, ImageFormat, MipRange};

const MAGIC: &[u8; 4] = b"DDS ";
const HEADER_SIZE: usize = 124;
const HEADER_DXT10_SIZE: usize = 20;
const PIXEL_FORMAT_SIZE: u32 = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DdsError {
    /// The data is too short to contain a DDS header.
    TooShort,
    /// The data does not start with the `DDS ` magic.
    BadMagic,
    /// The header contains invalid values (bad struct sizes, zero width/height, etc.).
    InvalidHeader,
    /// The header describes more pixel data than the file contains.
    Truncated,
    /// The DX10 header uses a DXGI format that is not supported.
    UnsupportedDxgiFormat(u32),
    /// The legacy header uses a FourCC that is not supported.
    UnsupportedFourCc([u8; 4]),
    /// The legacy header uses an uncompressed pixel layout that is not supported.
    UnsupportedPixelFormat,
    /// The file is a cubemap.
    Cubemap,
    /// The file is a texture array.
    TextureArray,
    /// The file is a volume (3D) texture.
    Volume,
    /// The file is not a 2D texture (e.g. 1D texture or buffer).
    NotTexture2D,
}

impl fmt::Display for DdsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooShort => write!(f, "data is too short to be a DDS file"),
            Self::BadMagic => write!(f, "missing DDS magic"),
            Self::InvalidHeader => write!(f, "invalid DDS header"),
            Self::Truncated => write!(f, "DDS file is truncated"),
            Self::UnsupportedDxgiFormat(format) => write!(f, "unsupported DXGI format {format}"),
            Self::UnsupportedFourCc(fourcc) => match core::str::from_utf8(fourcc) {
                Ok(s) => write!(f, "unsupported FourCC '{s}'"),
                Err(_) => write!(f, "unsupported FourCC {fourcc:?}"),
            },
            Self::UnsupportedPixelFormat => write!(f, "unsupported uncompressed pixel format"),
            Self::Cubemap => write!(f, "cubemaps are not supported"),
            Self::TextureArray => write!(f, "texture arrays are not supported"),
            Self::Volume => write!(f, "volume textures are not supported"),
            Self::NotTexture2D => write!(f, "only 2D textures are supported"),
        }
    }
}

#[repr(C)]
struct Header {
    size: u32,
    flags: u32,
    height: u32,
    width: u32,
    _pitch_lin_size: u32,
    depth: u32,
    mip_map_count: u32,
    _reserved: [u32; 11],
    format: PixelFormat,
    _caps: u32,
    caps2: u32,
    _caps3: u32,
    _caps4: u32,
    _reserved2: u32,
}

#[repr(C)]
struct HeaderDxt10 {
    format: u32,
    dims: u32,
    misc: u32,
    array_size: u32,
    _misc2: u32,
}

#[repr(C)]
struct PixelFormat {
    size: u32,
    flags: u32,
    fourcc: [u8; 4],
    rgb_bit_count: u32,
    r_mask: u32,
    g_mask: u32,
    b_mask: u32,
    a_mask: u32,
}

mod flags {
    pub const DDSD_DEPTH: u32 = 1 << 23;
}

mod pf_flags {
    pub const ALPHAPIXELS: u32 = 0x1;
    pub const ALPHA: u32 = 0x2;
    pub const FOURCC: u32 = 0x4;
    pub const RGB: u32 = 0x40;
    pub const LUMINANCE: u32 = 0x20000;
}

mod caps2 {
    pub const CUBEMAP: u32 = 0x200;
    pub const VOLUME: u32 = 0x200000;
}

mod format {
    pub const R8G8B8A8_UNORM: u32 = 28;
    pub const R8G8B8A8_UNORM_SRGB: u32 = 29;
    pub const R8_UNORM: u32 = 61;
    pub const A8_UNORM: u32 = 65;
    pub const BC1_UNORM: u32 = 71;
    pub const BC1_UNORM_SRGB: u32 = 72;
    pub const BC2_UNORM: u32 = 74;
    pub const BC2_UNORM_SRGB: u32 = 75;
    pub const BC3_UNORM: u32 = 77;
    pub const BC3_UNORM_SRGB: u32 = 78;
    pub const BC4_UNORM: u32 = 80;
    pub const BC4_SNORM: u32 = 81;
    pub const BC5_UNORM: u32 = 83;
    pub const BC5_SNORM: u32 = 84;
    pub const B8G8R8A8_UNORM: u32 = 87;
    pub const B8G8R8X8_UNORM: u32 = 88;
    pub const B8G8R8A8_UNORM_SRGB: u32 = 91;
    pub const B8G8R8X8_UNORM_SRGB: u32 = 93;
    pub const BC7_UNORM: u32 = 98;
    pub const BC7_UNORM_SRGB: u32 = 99;
}

mod dimensions {
    pub const TEXTURE2D: u32 = 3;
}

mod misc {
    pub const TEXTURECUBE: u32 = 0x4;
}

/// Reads a little-endian `u32` at `offset`. The caller guarantees the bounds.
fn read_u32(data: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]])
}

impl Header {
    fn parse(data: &[u8]) -> Self {
        debug_assert!(data.len() >= HEADER_SIZE);

        let mut reserved = [0u32; 11];
        for (i, r) in reserved.iter_mut().enumerate() {
            *r = read_u32(data, 28 + i * 4);
        }

        Self {
            size: read_u32(data, 0),
            flags: read_u32(data, 4),
            height: read_u32(data, 8),
            width: read_u32(data, 12),
            _pitch_lin_size: read_u32(data, 16),
            depth: read_u32(data, 20),
            mip_map_count: read_u32(data, 24),
            _reserved: reserved,
            format: PixelFormat::parse(&data[72..104]),
            _caps: read_u32(data, 104),
            caps2: read_u32(data, 108),
            _caps3: read_u32(data, 112),
            _caps4: read_u32(data, 116),
            _reserved2: read_u32(data, 120),
        }
    }
}

impl PixelFormat {
    fn parse(data: &[u8]) -> Self {
        Self {
            size: read_u32(data, 0),
            flags: read_u32(data, 4),
            fourcc: [data[8], data[9], data[10], data[11]],
            rgb_bit_count: read_u32(data, 12),
            r_mask: read_u32(data, 16),
            g_mask: read_u32(data, 20),
            b_mask: read_u32(data, 24),
            a_mask: read_u32(data, 28),
        }
    }

    fn has(&self, flag: u32) -> bool {
        self.flags & flag != 0
    }

    /// Maps a legacy (non-DX10) pixel format to an image format.
    fn to_image_format(&self) -> Result<(ImageFormat, ColorSpace), DdsError> {
        if self.has(pf_flags::FOURCC) {
            let format = match &self.fourcc {
                b"DXT1" => ImageFormat::Bc1,
                b"DXT2" | b"DXT3" => ImageFormat::Bc2,
                b"DXT4" | b"DXT5" => ImageFormat::Bc3,
                b"ATI1" | b"BC4U" => ImageFormat::Bc4,
                b"BC4S" => ImageFormat::Bc4Snorm,
                b"ATI2" | b"BC5U" => ImageFormat::Bc5,
                b"BC5S" => ImageFormat::Bc5Snorm,
                _ => return Err(DdsError::UnsupportedFourCc(self.fourcc)),
            };

            return Ok((format, default_color_space(format)));
        }

        let has_alpha = self.has(pf_flags::ALPHAPIXELS);
        let masks = (self.r_mask, self.g_mask, self.b_mask);

        let format = match self.rgb_bit_count {
            32 if self.has(pf_flags::RGB) => match masks {
                (0x0000_00FF, 0x0000_FF00, 0x00FF_0000) if has_alpha => ImageFormat::Rgba8,
                (0x0000_00FF, 0x0000_FF00, 0x00FF_0000) => ImageFormat::Rgbx8,
                (0x00FF_0000, 0x0000_FF00, 0x0000_00FF) if has_alpha => ImageFormat::Bgra8,
                (0x00FF_0000, 0x0000_FF00, 0x0000_00FF) => ImageFormat::Bgrx8,
                _ => return Err(DdsError::UnsupportedPixelFormat),
            },
            8 if self.has(pf_flags::ALPHA) && self.a_mask == 0xFF => ImageFormat::A8,
            8 if (self.has(pf_flags::LUMINANCE) || self.has(pf_flags::RGB)) && self.r_mask == 0xFF => ImageFormat::R8,
            _ => return Err(DdsError::UnsupportedPixelFormat),
        };

        Ok((format, default_color_space(format)))
    }
}

impl HeaderDxt10 {
    fn parse(data: &[u8]) -> Self {
        debug_assert!(data.len() >= HEADER_DXT10_SIZE);
        Self {
            format: read_u32(data, 0),
            dims: read_u32(data, 4),
            misc: read_u32(data, 8),
            array_size: read_u32(data, 12),
            _misc2: read_u32(data, 16),
        }
    }

    fn to_image_format(&self) -> Result<(ImageFormat, ColorSpace), DdsError> {
        use ColorSpace::{Linear, Srgb};

        let result = match self.format {
            format::R8G8B8A8_UNORM => (ImageFormat::Rgba8, Linear),
            format::R8G8B8A8_UNORM_SRGB => (ImageFormat::Rgba8, Srgb),
            format::R8_UNORM => (ImageFormat::R8, Linear),
            format::A8_UNORM => (ImageFormat::A8, Linear),
            format::BC1_UNORM => (ImageFormat::Bc1, Linear),
            format::BC1_UNORM_SRGB => (ImageFormat::Bc1, Srgb),
            format::BC2_UNORM => (ImageFormat::Bc2, Linear),
            format::BC2_UNORM_SRGB => (ImageFormat::Bc2, Srgb),
            format::BC3_UNORM => (ImageFormat::Bc3, Linear),
            format::BC3_UNORM_SRGB => (ImageFormat::Bc3, Srgb),
            format::BC4_UNORM => (ImageFormat::Bc4, Linear),
            format::BC4_SNORM => (ImageFormat::Bc4Snorm, Linear),
            format::BC5_UNORM => (ImageFormat::Bc5, Linear),
            format::BC5_SNORM => (ImageFormat::Bc5Snorm, Linear),
            format::B8G8R8A8_UNORM => (ImageFormat::Bgra8, Linear),
            format::B8G8R8A8_UNORM_SRGB => (ImageFormat::Bgra8, Srgb),
            format::B8G8R8X8_UNORM => (ImageFormat::Bgrx8, Linear),
            format::B8G8R8X8_UNORM_SRGB => (ImageFormat::Bgrx8, Srgb),
            format::BC7_UNORM => (ImageFormat::Bc7, Linear),
            format::BC7_UNORM_SRGB => (ImageFormat::Bc7, Srgb),
            other => return Err(DdsError::UnsupportedDxgiFormat(other)),
        };

        Ok(result)
    }
}

/// Legacy headers carry no color space information, except for formats that can only be linear.
fn default_color_space(format: ImageFormat) -> ColorSpace {
    match format {
        ImageFormat::Bc4 | ImageFormat::Bc4Snorm | ImageFormat::Bc5 | ImageFormat::Bc5Snorm => ColorSpace::Linear,
        ImageFormat::R8 | ImageFormat::A8 => ColorSpace::Linear,
        _ => ColorSpace::Unknown,
    }
}

/// Parses a DDS file. The returned image borrows its pixel data from `data`.
pub fn parse(data: &[u8]) -> Result<Image<'_>, DdsError> {
    if data.len() < MAGIC.len() + HEADER_SIZE {
        return Err(DdsError::TooShort);
    }
    if &data[..4] != MAGIC {
        return Err(DdsError::BadMagic);
    }

    let header = Header::parse(&data[4..4 + HEADER_SIZE]);
    if header.size as usize != HEADER_SIZE || header.format.size != PIXEL_FORMAT_SIZE {
        return Err(DdsError::InvalidHeader);
    }
    if header.width == 0 || header.height == 0 {
        return Err(DdsError::InvalidHeader);
    }

    let mut data_offset = MAGIC.len() + HEADER_SIZE;

    let (format, color_space) = if header.format.has(pf_flags::FOURCC) && &header.format.fourcc == b"DX10" {
        if data.len() < data_offset + HEADER_DXT10_SIZE {
            return Err(DdsError::TooShort);
        }

        let dx10 = HeaderDxt10::parse(&data[data_offset..data_offset + HEADER_DXT10_SIZE]);
        data_offset += HEADER_DXT10_SIZE;

        if dx10.misc & misc::TEXTURECUBE != 0 {
            return Err(DdsError::Cubemap);
        }
        if dx10.dims != dimensions::TEXTURE2D {
            return Err(if dx10.dims > dimensions::TEXTURE2D {
                DdsError::Volume
            } else {
                DdsError::NotTexture2D
            });
        }
        if dx10.array_size > 1 {
            return Err(DdsError::TextureArray);
        }

        dx10.to_image_format()?
    } else {
        if header.caps2 & caps2::CUBEMAP != 0 {
            return Err(DdsError::Cubemap);
        }
        if header.caps2 & caps2::VOLUME != 0 || (header.flags & flags::DDSD_DEPTH != 0 && header.depth > 1) {
            return Err(DdsError::Volume);
        }

        header.format.to_image_format()?
    };

    // Like DirectXTex, trust the mip count even if DDSD_MIPMAPCOUNT is not set; 0 means a single level.
    // Anything beyond the full chain is invalid.
    let max_mips = 32 - u32::max(header.width, header.height).leading_zeros();
    let mip_count = header.mip_map_count.max(1);
    if mip_count > max_mips {
        return Err(DdsError::InvalidHeader);
    }

    let mut mips = Vec::with_capacity(mip_count as usize);
    let mut offset = 0usize;
    for level in 0..mip_count {
        let width = (header.width >> level).max(1);
        let height = (header.height >> level).max(1);
        let length = format.surface_size(width, height).ok_or(DdsError::Truncated)?;

        mips.push(MipRange { offset, length });
        offset = offset.checked_add(length).ok_or(DdsError::Truncated)?;
    }

    let pixels = data_offset
        .checked_add(offset)
        .and_then(|end| data.get(data_offset..end))
        .ok_or(DdsError::Truncated)?;

    Ok(Image {
        width: header.width,
        height: header.height,
        format,
        color_space,
        data: Cow::Borrowed(pixels),
        mips,
    })
}
