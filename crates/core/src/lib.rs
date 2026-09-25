use std::io::Cursor;
use image::{DynamicImage, ImageFormat, RgbaImage};
use heic::{DecoderConfig, PixelLayout};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConvertError {
    #[error("Unsupported or unknown image format")]
    UnsupportedFormat,
    #[error("HEIC decoding failed: {0}")]
    HeicDecodeError(String),
    #[error("Image decoding failed: {0}")]
    ImageDecodeError(#[from] image::ImageError),
    #[error("Image encoding failed: {0}")]
    EncodeError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Png,
    Jpeg,
    WebP,
    Bmp,
    Tiff,
}

impl OutputFormat {
    pub fn to_image_format(self) -> ImageFormat {
        match self {
            OutputFormat::Png => ImageFormat::Png,
            OutputFormat::Jpeg => ImageFormat::Jpeg,
            OutputFormat::WebP => ImageFormat::WebP,
            OutputFormat::Bmp => ImageFormat::Bmp,
            OutputFormat::Tiff => ImageFormat::Tiff,
        }
    }

    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "png" => Some(OutputFormat::Png),
            "jpg" | "jpeg" => Some(OutputFormat::Jpeg),
            "webp" => Some(OutputFormat::WebP),
            "bmp" => Some(OutputFormat::Bmp),
            "tif" | "tiff" => Some(OutputFormat::Tiff),
            _ => None,
        }
    }
}

pub fn is_heic(bytes: &[u8]) -> bool {
    if bytes.len() < 12 {
        return false;
    }
    // Check for ftyp box
    &bytes[4..8] == b"ftyp"
}

pub fn decode_heic(bytes: &[u8]) -> Result<DynamicImage, ConvertError> {
    let output = DecoderConfig::new()
        .decode(bytes, PixelLayout::Rgba8)
        .map_err(|e| ConvertError::HeicDecodeError(format!("{:?}", e)))?;

    let img_buf = RgbaImage::from_raw(output.width, output.height, output.data)
        .ok_or_else(|| ConvertError::HeicDecodeError("Failed to create RgbaImage buffer".into()))?;

    Ok(DynamicImage::ImageRgba8(img_buf))
}
