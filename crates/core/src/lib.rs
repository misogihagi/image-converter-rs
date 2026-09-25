pub fn decode_heic(bytes: &[u8]) -> Result<DynamicImage, ConvertError> {
    let output = DecoderConfig::new()
        .decode(bytes, PixelLayout::Rgba8)
        .map_err(|e| ConvertError::HeicDecodeError(format!("{:?}", e)))?;

    let img_buf = RgbaImage::from_raw(output.width, output.height, output.data)
        .ok_or_else(|| ConvertError::HeicDecodeError("Failed to create RgbaImage buffer".into()))?;

    Ok(DynamicImage::ImageRgba8(img_buf))
}
