# image-converter-wasm

Pure Rust image converter WebAssembly package for Node.js and modern browsers.  
Decodes HEIC/HEIF without C libraries (`libheif` not needed) using [`imazen/heic`](https://github.com/imazen/heic), and converts between HEIC, JPEG, PNG, WebP, BMP, and TIFF.

## Features

- **Pure Rust / Pure WebAssembly**: No C toolchains or native system dependencies required.
- **Universal JavaScript / TypeScript Support**: Works out of the box in Node.js (CommonJS & ESM) and Browsers (Vite, Webpack, Next.js, Vanilla ESM).
- **TypeScript Ready**: Full type definitions included.
- **Supported Formats**:
  - **Input**: HEIC, HEIF, JPEG, PNG, WebP, BMP, TIFF, GIF
  - **Output**: JPEG, PNG, WebP, BMP, TIFF

## Installation

```bash
npm install image-converter-wasm
```

## Usage

### In Node.js (ESM / CommonJS)

In Node.js, the WebAssembly binary is automatically loaded synchronously. No explicit initialization is needed!

```javascript
import fs from "node:fs";
import { convert, detect_format, is_heic } from "image-converter-wasm";

// Read image file into Uint8Array
const inputBuffer = fs.readFileSync("photo.heic");

// Check format
console.log("Is HEIC:", is_heic(inputBuffer));
console.log("Format:", detect_format(inputBuffer)); // "heic"

// Convert HEIC to JPEG
const jpegBytes = convert(inputBuffer, "jpeg");
fs.writeFileSync("output.jpg", jpegBytes);

// Convert to PNG
const pngBytes = convert(inputBuffer, "png");
fs.writeFileSync("output.png", pngBytes);
```

### In Browser / Bundlers (Vite, Next.js, Webpack)

In browsers or web bundlers, initialize the WASM module first using `init()`:

```javascript
import init, { convert, detect_format, is_heic } from "image-converter-wasm";

async function run() {
  // 1. Initialize WASM module
  await init();

  // 2. Fetch or load image from <input type="file">
  const response = await fetch("/sample.heic");
  const arrayBuffer = await response.arrayBuffer();
  const inputBytes = new Uint8Array(arrayBuffer);

  // 3. Convert image
  console.log("Detected format:", detect_format(inputBytes));
  const convertedBytes = convert(inputBytes, "png");

  // 4. Create object URL for <img> preview or download
  const blob = new Blob([convertedBytes], { type: "image/png" });
  const imageUrl = URL.createObjectURL(blob);
  document.getElementById("myImage").src = imageUrl;
}

run();
```

## API Reference

### `convert(inputBytes: Uint8Array, format: string): Uint8Array`
Converts image bytes to target format (`"jpeg"`, `"jpg"`, `"png"`, `"webp"`, `"bmp"`, `"tiff"`). Throws an error on failure.

### `detect_format(inputBytes: Uint8Array): string`
Returns the detected image format: `"heic"`, `"jpeg"`, `"png"`, `"webp"`, `"bmp"`, `"tiff"`, or `"gif"`.

### `is_heic(inputBytes: Uint8Array): boolean`
Fast check if the input bytes start with a HEIC/HEIF `ftyp` box.

### `default init(moduleOrPath?: any): Promise<InitOutput>`
(Browser only) Loads and instantiates the WebAssembly module.

## License

MIT OR Apache-2.0
