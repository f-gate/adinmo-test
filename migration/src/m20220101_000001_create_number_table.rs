use sea_orm_migration::prelude::*;
use crate::m20231125_142413_create_post_batch_table::PostBatch;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RandomNumber::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RandomNumber::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RandomNumber::Number).integer().not_null())
                    .col(ColumnDef::new(RandomNumber::PostBatchId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-random-number-post-batch_id")
                            .from(RandomNumber::Table, RandomNumber::PostBatchId)
                            .to(PostBatch::Table, PostBatch::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RandomNumber::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum RandomNumber {
    Table,
    Id,
    Number,
    PostBatchId,
}
