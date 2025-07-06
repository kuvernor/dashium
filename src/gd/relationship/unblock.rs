use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{AppError, models::Block, util::verify_gjp2};

#[derive(Deserialize, Serialize, Debug)]
pub struct UnblockForm {
    accountID: i32,
    gjp2: String,
    targetAccountID: i32,
    gameVersion: i16,
    binaryVersion: i16,
    secret: String,
    udid: String,
    uuid: String,
}

pub async fn unblock_user(
    State(pool): State<PgPool>,
    Form(form): Form<UnblockForm>,
) -> Result<String, AppError> {
    let blocker_id = form.accountID;
    let gjp2 = &form.gjp2;
    let blocked_id = form.targetAccountID;

    if !verify_gjp2(&pool, blocker_id, gjp2).await? {
        return Ok("1".to_string());
    }

    if !Block::exists(&pool, blocker_id, blocked_id).await? {
        return Ok("1".to_string());
    }

    Block::unblock_user(&pool, blocker_id, blocked_id).await?;

    Ok("1".to_string())
}
