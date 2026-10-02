use dioxus::prelude::*;
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use image_converter_core::{convert_image, OutputFormat};

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[derive(Clone, Copy, PartialEq)]
enum TargetFormat {
    Png,
    Jpeg,
    Webp,
    Bmp,
    Tiff,
}

impl TargetFormat {
    fn label(&self) -> &'static str {
        match self {
            Self::Png => "PNG",
            Self::Jpeg => "JPEG",
            Self::Webp => "WebP",
            Self::Bmp => "BMP",
            Self::Tiff => "TIFF",
        }
    }

    fn extension(&self) -> &'static str {
        match self {
            Self::Png => "png",
            Self::Jpeg => "jpg",
            Self::Webp => "webp",
            Self::Bmp => "bmp",
            Self::Tiff => "tiff",
        }
    }

    fn mime_type(&self) -> &'static str {
        match self {
            Self::Png => "image/png",
            Self::Jpeg => "image/jpeg",
            Self::Webp => "image/webp",
            Self::Bmp => "image/bmp",
            Self::Tiff => "image/tiff",
        }
    }

    fn to_output_format(&self) -> OutputFormat {
        match self {
            Self::Png => OutputFormat::Png,
            Self::Jpeg => OutputFormat::Jpeg,
            Self::Webp => OutputFormat::WebP,
            Self::Bmp => OutputFormat::Bmp,
            Self::Tiff => OutputFormat::Tiff,
        }
    }
}

#[component]
fn App() -> Element {
    let mut file_name = use_signal(|| None::<String>);
    let mut file_bytes = use_signal(|| None::<Vec<u8>>);
    let mut target_format = use_signal(|| TargetFormat::Png);
    let mut converting = use_signal(|| false);
    let mut error_msg = use_signal(|| None::<String>);
    let mut converted_result = use_signal(|| None::<(String, String)>); // (download_filename, data_url)

    let handle_convert = move |_| {
        let bytes = file_bytes.read().clone();
        let fname = file_name.read().clone();
        let fmt = target_format();

        if let (Some(bytes), Some(fname)) = (bytes, fname) {
            converting.set(true);
            error_msg.set(None);
            converted_result.set(None);

            let res = convert_image(&bytes, fmt.to_output_format());
            converting.set(false);

            match res {
                Ok(out_bytes) => {
                    let base_name = std::path::Path::new(&fname)
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("image");
                    let out_filename = format!("{}.{}", base_name, fmt.extension());
                    let encoded = BASE64.encode(&out_bytes);
                    let data_url = format!("data:{};base64,{}", fmt.mime_type(), encoded);
                    converted_result.set(Some((out_filename, data_url)));
                }
                Err(err) => {
                    error_msg.set(Some(format!("変換に失敗しました: {}", err)));
                }
            }
        }
    };

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        div { class: "container",
            header {
                h1 { "🖼️ Image Converter" }
                p { "Pure Rust & WebAssembly によるクライアントサイド画像変換" }
            }

            div { class: "card",
                label { class: "dropzone",
                    input {
                        r#type: "file",
                        accept: ".heic,.heif,.jpg,.jpeg,.png,.webp,.bmp,.tiff",
                        onchange: move |evt| {
                            async move {
                                if let Some(file) = evt.files().into_iter().next() {
                                    let name = file.name();
                                    file_name.set(Some(name));
                                    converted_result.set(None);
                                    error_msg.set(None);

                                    match file.read_bytes().await {
                                        Ok(bytes) => {
                                            file_bytes.set(Some(bytes.to_vec()));
                                        }
                                        Err(err) => {
                                            error_msg.set(Some(format!("ファイルの読み込みに失敗しました: {}", err)));
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "dropzone-label",
                        span { class: "dropzone-icon", "📁" }
                        span { class: "dropzone-text", "画像ファイルを選択またはドロップ" }
                        span { class: "dropzone-hint", "HEIC, JPEG, PNG, WebP, BMP, TIFF に対応" }
                    }
                }

                if let Some(name) = file_name() {
                    div { class: "file-info",
                        span { "選択中: {name}" }
                        if let Some(bytes) = file_bytes() {
                            span { "{bytes.len() / 1024} KB" }
                        }
                    }
                }
            }

            div { class: "card",
                div { class: "section-title", "出力フォーマット" }
                div { class: "format-options",
                    for fmt in [
                        TargetFormat::Png,
                        TargetFormat::Jpeg,
                        TargetFormat::Webp,
                        TargetFormat::Bmp,
                        TargetFormat::Tiff,
                    ] {
                        button {
                            class: if target_format() == fmt { "format-btn active" } else { "format-btn" },
                            onclick: move |_| target_format.set(fmt),
                            "{fmt.label()}"
                        }
                    }
                }

                div { style: "margin-top: 1.5rem;",
                    button {
                        class: "action-btn",
                        disabled: file_bytes().is_none() || converting(),
                        onclick: handle_convert,
                        if converting() {
                            "⏳ 変換中..."
                        } else {
                            "変換する"
                        }
                    }
                }

                if let Some(err) = error_msg() {
                    div { class: "error-message", "{err}" }
                }
            }

            if let Some((download_name, data_url)) = converted_result() {
                div { class: "card result-card",
                    div { class: "section-title", "変換完了 🎉" }
                    div { class: "preview-container",
                        img {
                            class: "preview-image",
                            src: "{data_url}",
                            alt: "Converted image preview",
                        }
                    }
                    a {
                        class: "download-btn",
                        href: "{data_url}",
                        download: "{download_name}",
                        "画像をダウンロード ({download_name})"
                    }
                }
            }
        }
    }
}
