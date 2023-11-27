pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_number_table;
mod m20231125_142413_create_post_batch_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20231125_142413_create_post_batch_table::Migration),
            Box::new(m20220101_000001_create_number_table::Migration),
        ]
    }
}
