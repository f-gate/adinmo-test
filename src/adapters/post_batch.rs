use crate::entities::{prelude::PostBatch, *};
use sea_orm::*;

pub struct PostBatchAdapter;

impl PostBatchAdapter {
    pub async fn insert(db: &DatabaseConnection, time_to_query: String) -> Result<i32, DbErr> {
        let pb = post_batch::ActiveModel {
            time_to_query: ActiveValue::set(time_to_query),
            ..Default::default()
        };

        let res = PostBatch::insert(pb).exec(db).await?;
        Ok(res.last_insert_id)
    }
}
