use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ExpenseRecords::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ExpenseRecords::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ExpenseRecords::RecordType)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ExpenseRecords::CategoryId)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ExpenseRecords::Amount).decimal().not_null())
                    .col(ColumnDef::new(ExpenseRecords::Note).string())
                    .col(
                        ColumnDef::new(ExpenseRecords::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ExpenseRecords::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_expense_records_category")
                            .from(ExpenseRecords::Table, ExpenseRecords::CategoryId)
                            .to(Categories::Table, Categories::CategoryId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ExpenseRecords::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum ExpenseRecords {
    Table,
    Id,
    RecordType,
    CategoryId,
    Amount,
    Note,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Categories {
    Table,
    CategoryId,
}
