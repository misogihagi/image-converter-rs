use image_converter_core::{
    convert_image, detect_format as core_detect_format, is_heic as core_is_heic, OutputFormat,
};
use wasm_bindgen::prelude::*;

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

#[wasm_bindgen]
pub fn detect_format(input_bytes: &[u8]) -> Result<String, JsValue> {
    core_detect_format(input_bytes)
        .map(|s| s.to_string())
        .map_err(|e| JsValue::from_str(&e.to_string()))
}
