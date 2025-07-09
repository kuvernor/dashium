use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{AppError, util::verify_gjp2};

#[derive(Deserialize, Serialize, Debug)]
pub struct ReadForm {
    accountID: i32,
    gjp2: String,
    requestID: i32,
    gameVersion: i16,
    binaryVersion: i16,
    secret: String,
    udid: String,
    uuid: String,
}

pub async fn read_friend_request(
    State(pool): State<PgPool>,
    Form(form): Form<ReadForm>,
) -> Result<String, AppError> {
    let user_id = form.accountID;
    let gjp2 = &form.gjp2;
    let friend_request_id = form.requestID;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    sqlx::query!(
        "UPDATE friend_requests SET is_new = 0 WHERE id = $1",
        friend_request_id
    )
    .execute(&pool)
    .await?;

    Ok("1".to_string())
}
