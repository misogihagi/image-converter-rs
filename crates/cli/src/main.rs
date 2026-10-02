use std::fs;
use std::path::PathBuf;
use clap::Parser;
use image_converter_core::{convert_image, OutputFormat};

#[derive(Parser, Debug)]
#[command(author, version, about = "Pure Rust image converter CLI (HEIC, PNG, JPEG, WebP, etc.)")]
struct Args {
    /// Input file path
    input: PathBuf,

    /// Output file path
    output: PathBuf,

    /// Output format (png, jpg, webp, bmp, tiff). Inferred from output extension if omitted.
    #[arg(short, long)]
    format: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let output_format = if let Some(fmt_str) = &args.format {
        OutputFormat::from_extension(fmt_str)
            .ok_or_else(|| format!("Unsupported format: {}", fmt_str))?
    } else {
        let ext = args
            .output
            .extension()
            .and_then(|s| s.to_str())
            .ok_or("Could not infer output format from extension")?;
        OutputFormat::from_extension(ext)
            .ok_or_else(|| format!("Unsupported extension: {}", ext))?
    };

    println!("Reading input file: {}", args.input.display());
    let input_bytes = fs::read(&args.input)?;

    println!("Converting image...");
    let converted_bytes = convert_image(&input_bytes, output_format)?;

    println!("Writing output file: {}", args.output.display());
    fs::write(&args.output, converted_bytes)?;

    println!("Successfully converted!");
    Ok(())
}
