use crate::entities::{prelude::RandomNumber, *};
use sea_orm::*;

pub struct RandomNumberAdapter;

impl RandomNumberAdapter {
    pub async fn insert(
        db: &DatabaseConnection,
        number: i32,
        post_batch_id: i32,
    ) -> Result<i32, DbErr> {
        let rn = random_number::ActiveModel {
            number: ActiveValue::Set(number),
            post_batch_id: ActiveValue::Set(post_batch_id),
            ..Default::default()
        };

        let res = RandomNumber::insert(rn).exec(db).await?;
        Ok(res.last_insert_id)
    }
}
