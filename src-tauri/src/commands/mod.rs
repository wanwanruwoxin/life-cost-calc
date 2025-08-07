use sea_orm::DatabaseConnection;
use tauri::State;

use crate::database::{category_service, record_service};
use crate::entities::{category, expense_record};

pub type DbState<'a> = State<'a, DatabaseConnection>;

// Category Commands
#[tauri::command]
pub async fn get_categories(
    db: DbState<'_>,
    category_type: Option<String>,
) -> Result<Vec<category::Model>, String> {
    category_service::get_categories(&*db, category_type)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_category_by_id(
    db: DbState<'_>,
    category_id: String,
) -> Result<Option<category::Model>, String> {
    category_service::get_category_by_id(&*db, &category_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_category(
    db: DbState<'_>,
    category_data: category_service::CategoryData,
) -> Result<category::Model, String> {
    category_service::create_category(&*db, category_data)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_category(
    db: DbState<'_>,
    category_id: String,
    category_data: category_service::CategoryData,
) -> Result<category::Model, String> {
    category_service::update_category(&*db, &category_id, category_data)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_category(db: DbState<'_>, category_id: String) -> Result<(), String> {
    category_service::delete_category(&*db, &category_id)
        .await
        .map_err(|e| e.to_string())
}

// Record Commands
#[tauri::command]
pub async fn get_records(
    db: DbState<'_>,
    filter: Option<record_service::RecordFilter>,
) -> Result<Vec<expense_record::Model>, String> {
    record_service::get_records(&*db, filter)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_record_by_id(
    db: DbState<'_>,
    id: i32,
) -> Result<Option<expense_record::Model>, String> {
    record_service::get_record_by_id(&*db, id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_record(
    db: DbState<'_>,
    record_data: record_service::ExpenseRecordData,
) -> Result<expense_record::Model, String> {
    record_service::create_record(&*db, record_data)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_record(
    db: DbState<'_>,
    id: i32,
    record_data: record_service::ExpenseRecordData,
) -> Result<expense_record::Model, String> {
    record_service::update_record(&*db, id, record_data)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_record(db: DbState<'_>, id: i32) -> Result<(), String> {
    record_service::delete_record(&*db, id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_statistics(
    db: DbState<'_>,
    record_type: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
) -> Result<String, String> {
    let start_date = if let Some(date_str) = start_date {
        Some(
            chrono::DateTime::parse_from_rfc3339(&date_str)
                .map_err(|e| format!("Invalid start date: {}", e))?
                .with_timezone(&chrono::Utc),
        )
    } else {
        None
    };

    let end_date = if let Some(date_str) = end_date {
        Some(
            chrono::DateTime::parse_from_rfc3339(&date_str)
                .map_err(|e| format!("Invalid end date: {}", e))?
                .with_timezone(&chrono::Utc),
        )
    } else {
        None
    };

    let result = record_service::get_statistics(&*db, record_type, start_date, end_date)
        .await
        .map_err(|e| e.to_string())?;

    Ok(result.to_string())
}
