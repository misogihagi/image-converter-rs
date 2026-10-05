use heic::{DecoderConfig, PixelLayout};
use image::{DynamicImage, ImageFormat, RgbaImage};
use std::io::Cursor;
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

pub fn detect_format(bytes: &[u8]) -> Result<&'static str, ConvertError> {
    if is_heic(bytes) {
        return Ok("heic");
    }
    let format = image::guess_format(bytes).map_err(|_| ConvertError::UnsupportedFormat)?;
    match format {
        ImageFormat::Png => Ok("png"),
        ImageFormat::Jpeg => Ok("jpeg"),
        ImageFormat::WebP => Ok("webp"),
        ImageFormat::Bmp => Ok("bmp"),
        ImageFormat::Tiff => Ok("tiff"),
        ImageFormat::Gif => Ok("gif"),
        _ => Err(ConvertError::UnsupportedFormat),
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
    fn test_is_heic_valid() {
        let fake_heic = [0, 0, 0, 20, b'f', b't', b'y', b'p', b'h', b'e', b'i', b'c'];
        assert!(is_heic(&fake_heic));
    }

    #[test]
    fn test_decode_invalid_heic() {
        let res = decode_heic(b"invalid heic data");
        assert!(res.is_err());
    }

    #[test]
    fn test_output_format_extension() {
        assert_eq!(OutputFormat::from_extension("png"), Some(OutputFormat::Png));
        assert_eq!(
            OutputFormat::from_extension("jpg"),
            Some(OutputFormat::Jpeg)
        );
        assert_eq!(OutputFormat::from_extension("HEIC"), None);
    }

    #[test]
    fn test_detect_format() {
        let fake_heic = [0, 0, 0, 20, b'f', b't', b'y', b'p', b'h', b'e', b'i', b'c'];
        assert_eq!(detect_format(&fake_heic).unwrap(), "heic");

        let fake_png = [0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
        assert_eq!(detect_format(&fake_png).unwrap(), "png");

        let fake_jpeg = [
            0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, b'J', b'F', b'I', b'F', 0x00,
        ];
        assert_eq!(detect_format(&fake_jpeg).unwrap(), "jpeg");

        assert!(detect_format(b"unknown data").is_err());
    }
}
