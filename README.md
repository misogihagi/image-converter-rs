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
├── crates/
│   ├── core/           # 共通の変換ロジック（imazen/heic のラップ）
│   ├── cli/            # CLI インターフェース実装
│   └── wasm/           # wasm-bindgen を用いた WebAssembly インターフェース
└── examples/           # Webブラウザ（Wasm Web）での利用例（HTML/JS）
