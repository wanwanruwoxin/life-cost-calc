use rust_decimal::Decimal;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, QueryOrder,
    Set,
};
use serde::{Deserialize, Serialize};

use crate::entities::{expense_record, ExpenseRecord};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpenseRecordData {
    pub record_type: String,
    pub category_id: String,
    pub amount: Decimal,
    pub note: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordFilter {
    pub record_type: Option<String>,
    pub category_id: Option<String>,
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn get_records(
    db: &DatabaseConnection,
    filter: Option<RecordFilter>,
) -> Result<Vec<expense_record::Model>, DbErr> {
    let mut query = ExpenseRecord::find().order_by_desc(expense_record::Column::CreatedAt);

    if let Some(filter) = filter {
        if let Some(record_type) = filter.record_type {
            query = query.filter(expense_record::Column::RecordType.eq(record_type));
        }

        if let Some(category_id) = filter.category_id {
            query = query.filter(expense_record::Column::CategoryId.eq(category_id));
        }

        if let Some(start_date) = filter.start_date {
            query = query.filter(expense_record::Column::CreatedAt.gte(start_date));
        }

        if let Some(end_date) = filter.end_date {
            query = query.filter(expense_record::Column::CreatedAt.lte(end_date));
        }
    }

    query.all(db).await
}

pub async fn get_record_by_id(
    db: &DatabaseConnection,
    id: i32,
) -> Result<Option<expense_record::Model>, DbErr> {
    ExpenseRecord::find_by_id(id).one(db).await
}

pub async fn create_record(
    db: &DatabaseConnection,
    record_data: ExpenseRecordData,
) -> Result<expense_record::Model, DbErr> {
    let now = chrono::Utc::now().into();
    let record = expense_record::ActiveModel {
        record_type: Set(record_data.record_type),
        category_id: Set(record_data.category_id),
        amount: Set(record_data.amount),
        note: Set(record_data.note),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    record.insert(db).await
}

pub async fn update_record(
    db: &DatabaseConnection,
    id: i32,
    record_data: ExpenseRecordData,
) -> Result<expense_record::Model, DbErr> {
    let record: Option<expense_record::Model> = ExpenseRecord::find_by_id(id).one(db).await?;

    if let Some(record) = record {
        let mut record: expense_record::ActiveModel = record.into();
        record.record_type = Set(record_data.record_type);
        record.category_id = Set(record_data.category_id);
        record.amount = Set(record_data.amount);
        record.note = Set(record_data.note);
        record.updated_at = Set(chrono::Utc::now().into());

        record.update(db).await
    } else {
        Err(DbErr::RecordNotFound("Record not found".into()))
    }
}

pub async fn delete_record(db: &DatabaseConnection, id: i32) -> Result<(), DbErr> {
    ExpenseRecord::delete_by_id(id).exec(db).await?;
    Ok(())
}

// 获取统计数据
pub async fn get_statistics(
    db: &DatabaseConnection,
    record_type: Option<String>,
    start_date: Option<chrono::DateTime<chrono::Utc>>,
    end_date: Option<chrono::DateTime<chrono::Utc>>,
) -> Result<Decimal, DbErr> {
    use sea_orm::{FromQueryResult, QuerySelect};

    #[derive(Debug, FromQueryResult)]
    struct SumResult {
        total: Option<Decimal>,
    }

    let mut query = ExpenseRecord::find()
        .select_only()
        .column_as(expense_record::Column::Amount.sum(), "total");

    if let Some(record_type) = record_type {
        query = query.filter(expense_record::Column::RecordType.eq(record_type));
    }

    if let Some(start_date) = start_date {
        query = query.filter(expense_record::Column::CreatedAt.gte(start_date));
    }

    if let Some(end_date) = end_date {
        query = query.filter(expense_record::Column::CreatedAt.lte(end_date));
    }

    let result: Option<SumResult> = query.into_model::<SumResult>().one(db).await?;

    Ok(result
        .and_then(|r| r.total)
        .unwrap_or_else(|| Decimal::from(0)))
}
