use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "categories")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub category_id: String,
    pub name: String,
    pub icon: String,
    pub color: String,
    pub category_type: String, // "expense" or "income"
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::expense_record::Entity")]
    ExpenseRecord,
}

impl Related<super::expense_record::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ExpenseRecord.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
