use std::path::PathBuf;
use tauri::plugin::TauriPlugin;
use tauri::{Manager, Runtime};
use tauri_plugin_log::fern::colors::Color;
use tauri_plugin_log::fern::colors::ColoredLevelConfig;
use tauri_plugin_log::{Target, TargetKind, WEBVIEW_TARGET};

mod commands;
mod database;
mod entities;
mod migration;

#[cfg(desktop)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(build_log_plugin())
        .setup(|app| {
            // 初始化数据库
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                match database::establish_connection().await {
                    Ok(db) => {
                        // 初始化默认分类数据
                        if let Err(e) =
                            database::category_service::initialize_default_categories(&db).await
                        {
                            log::error!("Failed to initialize default categories: {}", e);
                        } else {
                            log::info!("Database initialized successfully");
                        }

                        // 将数据库连接保存到应用状态
                        app_handle.manage(db);
                    }
                    Err(e) => {
                        log::error!("Failed to establish database connection: {}", e);
                    }
                }
            });

            // 创建无边框窗口
            use tauri::WebviewWindowBuilder;
            let _window =
                WebviewWindowBuilder::new(app, "main", tauri::WebviewUrl::App("index.html".into()))
                    .title("羊羊的记账本")
                    .inner_size(390.0, 844.0)
                    .min_inner_size(375.0, 667.0)
                    .max_inner_size(430.0, 932.0)
                    .resizable(true)
                    .center()
                    .decorations(false) // 去掉原生窗口装饰
                    .always_on_top(false)
                    .skip_taskbar(false)
                    .theme(Some(tauri::Theme::Light))
                    .build()?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_categories,
            commands::get_category_by_id,
            commands::create_category,
            commands::update_category,
            commands::delete_category,
            commands::get_records,
            commands::get_record_by_id,
            commands::create_record,
            commands::update_record,
            commands::delete_record,
            commands::get_statistics
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
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
