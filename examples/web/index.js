import init, { convert, detect_format, is_heic } from "./dist/web/image_converter_wasm.js";

const fileInput = document.getElementById("fileInput");
const fileInfo = document.getElementById("fileInfo");
const formatSelect = document.getElementById("formatSelect");
const convertBtn = document.getElementById("convertBtn");
const statusDiv = document.getElementById("status");
const resultCard = document.getElementById("resultCard");
const previewImage = document.getElementById("previewImage");
const downloadLink = document.getElementById("downloadLink");

let selectedFileBytes = null;
let selectedFileName = "";

async function main() {
  statusDiv.textContent = "WebAssembly モジュールを読み込み中...";
  try {
    await init();
    statusDiv.textContent = "WASM 初期化完了。画像を選択してください。";
  } catch (err) {
    statusDiv.textContent = `WASM 初期化エラー: ${err}`;
    console.error(err);
    return;
  }

  fileInput.addEventListener("change", async (e) => {
    const file = e.target.files[0];
    if (!file) {
      selectedFileBytes = null;
      convertBtn.disabled = true;
      fileInfo.style.display = "none";
      return;
    }

    selectedFileName = file.name;
    const arrayBuffer = await file.arrayBuffer();
    selectedFileBytes = new Uint8Array(arrayBuffer);

    let detected = "不明";
    try {
      detected = detect_format(selectedFileBytes);
    } catch {
      detected = is_heic(selectedFileBytes) ? "heic" : "不明";
    }

    fileInfo.textContent = `選択中: ${file.name} (${(file.size / 1024).toFixed(1)} KB) - 推定形式: ${detected.toUpperCase()}`;
    fileInfo.style.display = "inline-block";
    convertBtn.disabled = false;
    statusDiv.textContent = "変換の準備が整いました。";
  });

  convertBtn.addEventListener("click", () => {
    if (!selectedFileBytes) return;

    const targetFormat = formatSelect.value;
    statusDiv.textContent = `${targetFormat.toUpperCase()} 形式へ変換中...`;
    convertBtn.disabled = true;

    // Use setTimeout so UI updates before heavy conversion
    setTimeout(() => {
      const startTime = performance.now();
      try {
        const convertedBytes = convert(selectedFileBytes, targetFormat);
        const elapsed = (performance.now() - startTime).toFixed(1);

        const mimeTypes = {
          jpeg: "image/jpeg",
          png: "image/png",
          webp: "image/webp",
          bmp: "image/bmp",
          tiff: "image/tiff",
        };
        const mime = mimeTypes[targetFormat] || "application/octet-stream";
        const blob = new Blob([convertedBytes], { type: mime });
        const url = URL.createObjectURL(blob);

        previewImage.src = url;
        const baseName = selectedFileName.replace(/\.[^/.]+$/, "");
        const ext = targetFormat === "jpeg" ? "jpg" : targetFormat;
        downloadLink.href = url;
        downloadLink.download = `${baseName}_converted.${ext}`;

        resultCard.style.display = "block";
        statusDiv.textContent = `変換完了！ (${elapsed}ms, 出力サイズ: ${(convertedBytes.length / 1024).toFixed(1)} KB)`;
      } catch (err) {
        statusDiv.textContent = `変換失敗: ${err}`;
        console.error("Conversion error:", err);
      } finally {
        convertBtn.disabled = false;
      }
    }, 20);
  });
}

main();
