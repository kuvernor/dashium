use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::AppError;
use crate::GDResponse;
use crate::models::{Block, User};
use crate::util::verify_gjp2;

#[derive(Serialize, Deserialize, Debug)]
pub struct InfoForm {
    accountID: i32,
    targetAccountID: i32,
    gjp2: String,
    gameVersion: i16,
    binaryVersion: i16,
    secret: String,
}

pub async fn info(
    State(pool): State<PgPool>,
    Form(form): Form<InfoForm>,
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
    let response = user.to_gd();

    Ok(response)
}
