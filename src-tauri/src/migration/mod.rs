pub use sea_orm_migration::prelude::*;

mod m20231212_000001_create_categories_table;
mod m20231212_000002_create_expense_records_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20231212_000001_create_categories_table::Migration),
            Box::new(m20231212_000002_create_expense_records_table::Migration),
        ]
    }
}
