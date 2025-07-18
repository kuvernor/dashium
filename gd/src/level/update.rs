use axum::{Form, extract::State};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    AppError,
    util::{base64_decode, verify_gjp2},
};

#[derive(Deserialize)]
pub struct updateGJDesc20 {
    accountID: i32,
    gjp2: String,
    levelDesc: String,
    levelID: i32,
}

pub async fn updateGJDesc20(
    State(pool): State<PgPool>,
    Form(form): Form<updateGJDesc20>,
) -> Result<String, AppError> {
    let user_id = form.accountID;
    let gjp2 = &form.gjp2;
    let level_id = form.levelID;

    let description = String::from_utf8(base64_decode(&form.levelDesc)?)?;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    sqlx::query!(
        "UPDATE levels SET description = $1 WHERE id = $2 AND user_id = $3",
        description,
        level_id,
        user_id
    )
    .execute(&pool)
    .await?;

    Ok("1".to_string())
}
