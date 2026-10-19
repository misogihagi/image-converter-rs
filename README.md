# image-converter-rs

`image-converter-rs` は、純粋な Rust 実装の HEIC デコーダである [`imazen/heic`](https://github.com/imazen/heic) をコアに採用した、高速かつポータブルな画像変換ツールです。

C言語の外部ライブラリ（`libheic` など）に依存しないため、クロスコンパイルが非常に容易であり、ネイティブの CLI 環境からブラウザ（WebAssembly）環境まで、同一のコードベースでシームレスに動作します。

## 🚀 特徴

- **純粋 Rust 実装 (Pure Rust):** `imazen/heic` を使用しているため、Cのランタイムや複雑なリンクエラーに悩まされることなく、安全かつ簡単にビルド可能です。
- **マルチプラットフォーム対応:**
  - **CLI:** ターミナルから直接、単一または複数の HEIC 形式の画像を JPEG/PNG へ高速変換。
  - **Wasm (Bundler/Node.js):** npm パッケージとして、サーバーサイドやフロントエンドのビルドパイプラインに組み込み可能。
  - **Wasm Web:** 外部サーバーに依存せず、ブラウザ上で直接（クライアントサイドで）画像のローカル変換を実現。

## 📂 プロジェクト構成

Cargo ワークスペースを利用して、コアロジックと各インターフェースを分離しています。

```text
image-converter-rs/
├── Cargo.toml          # ワークスペース管理
├── package.json        # ルート npm 設定・ビルドスクリプト
├── crates/
│   ├── core/           # 共通の変換ロジック（imazen/heic のラップ）
│   ├── cli/            # CLI インターフェース実装
│   └── wasm/           # wasm-bindgen を用いた WebAssembly インターフェース
├── packages/
│   └── image-converter-wasm/  # JavaScript / TypeScript 向け npm パッケージ
├── examples/
│   ├── node/           # Node.js での利用例 (convert.mjs)
│   └── web/            # ブラウザでの利用例 (index.html, index.js)
└── docs/
    └── specification.md # 仕様書
```

## 📦 WebAssembly (JavaScript / TypeScript) パッケージ

### ビルド

```bash
# 全体ビルド（Node.js 向け + Web 向けの Universal パッケージ生成）
npm run build

# Node.js 動作テスト
npm run test:node
```

### Node.js での利用例

```javascript
import fs from "node:fs";
import { convert, detect_format, is_heic } from "image-converter-wasm";

const buffer = fs.readFileSync("photo.heic");
console.log("Format:", detect_format(buffer)); // "heic"

const jpegBytes = convert(buffer, "jpeg");
fs.writeFileSync("output.jpg", jpegBytes);
```

### ブラウザ（Vite / Webpack / Vanilla）での利用例

```javascript
import init, { convert, detect_format } from "image-converter-wasm";

await init(); // WebAssembly モジュールの初期化

const res = await fetch("/photo.heic");
const buffer = new Uint8Array(await res.arrayBuffer());

const pngBytes = convert(buffer, "png");
const blob = new Blob([pngBytes], { type: "image/png" });
const url = URL.createObjectURL(blob);
```
