use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::AppError;
use crate::GDResponse;
use crate::models::{Block, User};
use crate::util::verify_gjp2;

#[derive(Serialize, Deserialize, Debug)]
pub struct getGJUserInfo20 {
    accountID: i32,
    targetAccountID: i32,
    gjp2: String,
    gameVersion: i16,
    binaryVersion: i16,
    secret: String,
}

pub async fn getGJUserInfo20(
    State(pool): State<PgPool>,
    Form(form): Form<getGJUserInfo20>,
) -> Result<String, AppError> {
    let user_id = form.accountID;
    let target_id = form.targetAccountID;
    let gjp2 = &form.gjp2;

    if Block::is_blocked(&pool, target_id, user_id).await?
        && verify_gjp2(&pool, user_id, gjp2).await?
    {
        return Ok("-1".to_string());
    }

    let user = User::get_user(&pool, target_id).await?;
    let mut response = user.to_gd();

    let new_messages = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM MESSAGES WHERE recipient_id = $1 AND is_read = false",
        user_id
    )
    .fetch_one(&pool)
    .await?
    .unwrap_or(0);

    let new_messages = format!("38:{new_messages}:");
    response.insert_str(0, &new_messages);

    Ok(response)
}
