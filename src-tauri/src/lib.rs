use tauri::plugin::TauriPlugin;
use tauri::{Runtime, WebviewWindowBuilder};
use tauri_plugin_log::fern::colors::Color;
use tauri_plugin_log::fern::colors::ColoredLevelConfig;
use tauri_plugin_log::{Target, TargetKind, WEBVIEW_TARGET};

#[cfg(desktop)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(build_log_plugin())
        .setup(|app| {
            // 创建符合iOS风格的窗口
            let _window = WebviewWindowBuilder::new(app, "main", tauri::WebviewUrl::App("index.html".into()))
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
        .invoke_handler(tauri::generate_handler![initialize_app])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn initialize_app() {
    log::error!("something bad happened!");
    log::info!("Tauri is awesome!");
}

fn build_log_plugin<R: Runtime>() -> TauriPlugin<R> {
    tauri_plugin_log::Builder::new()
        .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
        .level(log::LevelFilter::Info)
        .targets([
            Target::new(TargetKind::Stdout),
            Target::new(TargetKind::Webview),
            Target::new(TargetKind::LogDir { file_name: None })
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
