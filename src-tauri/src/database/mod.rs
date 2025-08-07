use sea_orm::{Database, DatabaseConnection, DbErr};
use sea_orm_migration::prelude::*;
use std::path::PathBuf;

use crate::migration::Migrator;

pub mod category_service;
pub mod record_service;

pub async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
    // 获取应用数据目录
    let mut db_path = get_app_data_dir();
    db_path.push("database.sqlite");

    // 确保目录存在
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| DbErr::Custom(format!("Failed to create database directory: {}", e)))?;
    }

    let database_url = format!("sqlite://{}?mode=rwc", db_path.display());

    let db = Database::connect(&database_url).await?;

    // 运行迁移
    Migrator::up(&db, None).await?;

    Ok(db)
}

fn get_app_data_dir() -> PathBuf {
    let app_name = "life-cost-calc";

    #[cfg(target_os = "windows")]
    {
        let mut path = std::env::var("APPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("."));
        path.push(app_name);
        path
    }

    #[cfg(target_os = "macos")]
    {
        let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("Library");
        path.push("Application Support");
        path.push(app_name);
        path
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        let mut path = std::env::var("XDG_DATA_HOME")
            .map(PathBuf::from)
            .or_else(|_| {
                dirs::home_dir().map(|mut home| {
                    home.push(".local");
                    home.push("share");
                    home
                })
            })
            .unwrap_or_else(|| PathBuf::from("."));
        path.push(app_name);
        path
    }
}
