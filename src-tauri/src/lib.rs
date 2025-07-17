use tauri::plugin::TauriPlugin;
use tauri::Runtime;
use tauri_plugin_log::fern::colors::Color;
use tauri_plugin_log::fern::colors::ColoredLevelConfig;
use tauri_plugin_log::{Target, TargetKind, WEBVIEW_TARGET};

#[cfg(desktop)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(build_log_plugin())
        .setup(|_app| Ok(()))
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
