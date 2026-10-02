use wasm_bindgen::prelude::*;
use image_converter_core::{convert_image, is_heic as core_is_heic, OutputFormat};

#[wasm_bindgen]
pub fn convert(input_bytes: &[u8], format: &str) -> Result<Vec<u8>, JsValue> {
    let output_format = OutputFormat::from_extension(format)
        .ok_or_else(|| JsValue::from_str(&format!("Unsupported format: {}", format)))?;

    convert_image(input_bytes, output_format)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn is_heic(input_bytes: &[u8]) -> bool {
    core_is_heic(input_bytes)
}
