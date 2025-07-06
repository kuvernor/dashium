use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{AppError, util::verify_gjp2};

#[derive(Deserialize, Serialize, Debug)]
pub struct RequestForm {
    accountID: i32,
    gjp2: String,
    gameVersion: i16,
    binaryVersion: i16,
    secret: String,
    udid: String,
    uuid: String,
}

pub async fn request_moderator(
    State(pool): State<PgPool>,
    Form(form): Form<RequestForm>,
) -> Result<String, AppError> {
    let user_id = form.accountID;
    let gjp2 = &form.gjp2;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    let mod_level = sqlx::query_scalar!("SELECT mod_level FROM users WHERE user_id = $1", user_id)
        .fetch_one(&pool)
        .await?;

    match mod_level {
        1 => Ok("1".to_string()),
        2 => Ok("2".to_string()),
        _ => Ok("-1".to_string()),
    }
}
