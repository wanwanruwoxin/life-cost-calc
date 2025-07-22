use std::path::PathBuf;
use tauri::plugin::TauriPlugin;
use tauri::Runtime;
use tauri_plugin_log::fern::colors::Color;
use tauri_plugin_log::fern::colors::ColoredLevelConfig;
use tauri_plugin_log::{Target, TargetKind, WEBVIEW_TARGET};

#[cfg(desktop)]
pub fn run() {
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(build_log_plugin())
        .setup(|app| {
            // 创建符合iOS风格的窗口

            use tauri::WebviewWindowBuilder;
            let _window =
                WebviewWindowBuilder::new(app, "main", tauri::WebviewUrl::App("index.html".into()))
                    .title("羊羊的记账本")
                    .inner_size(390.0, 844.0)
                    .min_inner_size(375.0, 667.0)
                    .max_inner_size(430.0, 932.0)
                    .resizable(true)
                    .center()
                    .decorations(true)
                    .always_on_top(false)
                    .skip_taskbar(false)
                    .theme(Some(tauri::Theme::Light))
                    .build()?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![initialize_app]);

    #[cfg(debug_assertions)]
    {
        builder = builder
            .plugin(tauri_plugin_devtools::init())
            .plugin(tauri_plugin_devtools_app::init());
    }

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn initialize_app() {
    log::error!("something bad happened!");
    log::info!("Tauri is awesome!");
}

fn build_log_plugin<R: Runtime>() -> TauriPlugin<R> {
    // 获取当前工作目录并创建 logs 子目录
    let mut log_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    log_dir.push("logs");

    // 确保 logs 目录存在
    if !log_dir.exists() {
        std::fs::create_dir_all(&log_dir).unwrap_or_else(|e| {
            eprintln!("Failed to create logs directory: {}", e);
        });
    }

    tauri_plugin_log::Builder::new()
        .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
        .skip_logger()
        .level(log::LevelFilter::Info)
        .targets([
            Target::new(TargetKind::Stdout),
            // 将 rust 日志打印到 webview的 devtool 中
            Target::new(TargetKind::Webview),
            // 将日志保存到项目的 logs 目录下
            Target::new(TargetKind::Folder {
                path: log_dir,
                file_name: Some("app".into()),
            })
            .filter(|metadata| !metadata.target().starts_with(WEBVIEW_TARGET)),
        ])
        .with_colors(ColoredLevelConfig {
            error: Color::Red,
            warn: Color::Yellow,
            debug: Color::White,
            info: Color::Green,
            trace: Color::White,
        })
        .build()
}
