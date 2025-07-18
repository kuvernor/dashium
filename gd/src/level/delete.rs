use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{AppError, models::Level, util::verify_gjp2};

#[derive(Deserialize, Serialize, Debug)]
pub struct deleteGJLevelUser20 {
    accountID: i32,
    gjp2: String,
    levelID: i32,
    binaryVersion: i16,
    gameVersion: i16,
    secret: String,
    udid: String,
    uuid: String,
}

pub async fn deleteGJLevelUser20(
    State(pool): State<PgPool>,
    Form(form): Form<deleteGJLevelUser20>,
) -> Result<String, AppError> {
    let user_id = form.accountID;
    let gjp2 = &form.gjp2;
    let level_id = form.levelID;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    Level::delete(&pool, user_id, level_id).await?;

    Ok("1".to_string())
}
