use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{AppError, util::verify_gjp2};

#[derive(Deserialize, Serialize, Debug)]
pub struct ReadForm {
    #[serde(rename = "accountID")]
    user_id: i32,

    gjp2: String,

    #[serde(rename = "requestID")]
    friend_request_id: i32,

    #[serde(rename = "gameVersion")]
    game_version: i16,

    #[serde(rename = "binaryVersion")]
    binary_version: i16,

    secret: String,
    udid: String,
    uuid: String,
}

pub async fn read_friend_request(
    State(pool): State<PgPool>,
    Form(form): Form<ReadForm>,
) -> Result<String, AppError> {
    let user_id = form.user_id;
    let gjp2 = &form.gjp2;
    let friend_request_id = form.friend_request_id;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    sqlx::query!(
        "UPDATE friend_requests SET is_new = 0 WHERE friend_request_id = $1",
        friend_request_id
    )
    .execute(&pool)
    .await?;

    Ok("1".to_string())
}
