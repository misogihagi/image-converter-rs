import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

// Import from the built npm package
import { convert, detect_format, is_heic } from "../../packages/image-converter-wasm/dist/node/image_converter_wasm.js";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDir = path.resolve(__dirname, "../..");

const inputPath = path.join(rootDir, "IMG_0383.HEIC");
const outputJpgPath = path.join(__dirname, "output.jpg");
const outputPngPath = path.join(__dirname, "output.png");

console.log("📷 Testing Node.js image conversion using image-converter-wasm...");
console.log(`Reading input file: ${inputPath}`);

if (!fs.existsSync(inputPath)) {
  console.error(`❌ Input file not found: ${inputPath}`);
  process.exit(1);
}

const inputBuffer = fs.readFileSync(inputPath);
console.log(`Input file size: ${inputBuffer.length.toLocaleString()} bytes`);

// 1. Check HEIC detection
const heicCheck = is_heic(inputBuffer);
console.log(`is_heic check: ${heicCheck}`);
if (!heicCheck) {
  console.error("❌ Expected is_heic to return true");
  process.exit(1);
}

// 2. Check format detection
const detectedFormat = detect_format(inputBuffer);
console.log(`detect_format: "${detectedFormat}"`);
if (detectedFormat !== "heic") {
  console.error(`❌ Expected format to be "heic", got "${detectedFormat}"`);
  process.exit(1);
}

// 3. Convert to JPEG
console.log("\nConverting HEIC to JPEG...");
const startTimeJpg = performance.now();
const jpegBytes = convert(inputBuffer, "jpeg");
const elapsedJpg = (performance.now() - startTimeJpg).toFixed(1);
console.log(`JPEG converted in ${elapsedJpg}ms, size: ${jpegBytes.length.toLocaleString()} bytes`);

// Verify JPEG magic bytes (0xFF, 0xD8)
if (jpegBytes[0] !== 0xFF || jpegBytes[1] !== 0xD8) {
  console.error("❌ Invalid JPEG magic numbers:", jpegBytes.slice(0, 4));
  process.exit(1);
}
fs.writeFileSync(outputJpgPath, jpegBytes);
console.log(`Saved JPEG to: ${outputJpgPath}`);

// 4. Convert to PNG
console.log("\nConverting HEIC to PNG...");
const startTimePng = performance.now();
const pngBytes = convert(inputBuffer, "png");
const elapsedPng = (performance.now() - startTimePng).toFixed(1);
console.log(`PNG converted in ${elapsedPng}ms, size: ${pngBytes.length.toLocaleString()} bytes`);

// Verify PNG magic bytes (0x89, 0x50, 0x4E, 0x47)
if (
  pngBytes[0] !== 0x89 ||
  pngBytes[1] !== 0x50 ||
  pngBytes[2] !== 0x4E ||
  pngBytes[3] !== 0x47
) {
  console.error("❌ Invalid PNG magic numbers:", pngBytes.slice(0, 4));
  process.exit(1);
}
fs.writeFileSync(outputPngPath, pngBytes);
console.log(`Saved PNG to: ${outputPngPath}`);

console.log("\n🎉 All Node.js tests passed successfully!");
