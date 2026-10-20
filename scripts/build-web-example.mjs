import { execSync } from "child_process";
import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDir = path.resolve(__dirname, "..");
const exampleDir = path.join(rootDir, "examples", "web");
const wasmWebDir = path.join(rootDir, "packages", "image-converter-wasm", "dist", "web");
const outDir = path.join(rootDir, "dist", "web-example");

const skipWasm = process.argv.includes("--skip-wasm");

if (!skipWasm) {
  console.log("🔨 Building WebAssembly (wasm-pack --target web)...");
  execSync("npm run build:wasm:web", { stdio: "inherit", cwd: rootDir });
} else {
  console.log("⏭️  Skipping WASM build (--skip-wasm)");
}

if (!fs.existsSync(path.join(wasmWebDir, "image_converter_wasm_bg.wasm"))) {
  console.error(`❌ WASM output not found: ${wasmWebDir}`);
  console.error("   Run `npm run build:wasm:web` first.");
  process.exit(1);
}

console.log("📦 Assembling static site...");
fs.rmSync(outDir, { recursive: true, force: true });
fs.mkdirSync(outDir, { recursive: true });

fs.copyFileSync(path.join(exampleDir, "index.html"), path.join(outDir, "index.html"));
fs.copyFileSync(path.join(exampleDir, "index.js"), path.join(outDir, "index.js"));

// index.js imports "./dist/web/image_converter_wasm.js"
fs.cpSync(wasmWebDir, path.join(outDir, "dist", "web"), { recursive: true });

// wasm-pack metadata is irrelevant for the static site
for (const file of ["package.json", ".gitignore"]) {
  const target = path.join(outDir, "dist", "web", file);
  if (fs.existsSync(target)) fs.unlinkSync(target);
}

console.log(`\n✅ Static site generated: ${outDir}`);
console.log("   Preview: npx serve dist/web-example");
