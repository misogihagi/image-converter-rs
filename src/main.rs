use std::env;
use std::fs;
use image::{DynamicImage, RgbaImage};
use heic::{DecoderConfig, PixelLayout};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: image-converter-rs <input.heic> <output.png>");
        return Ok(());
    }

    let input_path = &args[1];
    let output_path = &args[2];

    println!("Reading HEIC file: {}", input_path);
    let bytes = fs::read(input_path)?;

    println!("Decoding HEIC...");
    let dynamic_img = decode_heic(&bytes)?;

    println!("Saving to PNG: {}", output_path);
    dynamic_img.save(output_path)?;

    println!("Successfully converted {} to {}", input_path, output_path);
    Ok(())
}

fn decode_heic(bytes: &[u8]) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    let output = DecoderConfig::new()
        .decode(bytes, PixelLayout::Rgba8)
        .map_err(|e| format!("HEIC decode error: {:?}", e))?;

    let img_buf = RgbaImage::from_raw(output.width, output.height, output.data)
        .ok_or("Failed to create RgbaImage buffer")?;

    Ok(DynamicImage::ImageRgba8(img_buf))
}

