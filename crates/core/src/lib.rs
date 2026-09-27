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

pub fn decode_image(bytes: &[u8]) -> Result<DynamicImage, ConvertError> {
    if is_heic(bytes) {
        decode_heic(bytes)
    } else {
        let img = image::load_from_memory(bytes)?;
        Ok(img)
    }
}

pub fn convert_image(
    input_bytes: &[u8],
    output_format: OutputFormat,
) -> Result<Vec<u8>, ConvertError> {
    let img = decode_image(input_bytes)?;
    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);

    img.write_to(&mut cursor, output_format.to_image_format())
        .map_err(|e| ConvertError::EncodeError(e.to_string()))?;

    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_heic_invalid() {
        assert!(!is_heic(b"not a heic file"));
    }

    #[test]
    fn test_decode_invalid_heic() {
        let res = decode_heic(b"invalid heic data");
        assert!(res.is_err());
    }

    #[test]
    fn test_output_format_extension() {
        assert_eq!(OutputFormat::from_extension("png"), Some(OutputFormat::Png));
        assert_eq!(OutputFormat::from_extension("jpg"), Some(OutputFormat::Jpeg));
        assert_eq!(OutputFormat::from_extension("HEIC"), None);
    }
}
