use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{AppError, models::User, util::verify_gjp2};

#[derive(Deserialize, Serialize, Debug)]
pub struct SaveForm {
    accountID: i32,
    gjp2: String,
    gameVersion: i16,
    binaryVersion: i16,
    saveData: String,
    secret: String,
    udid: String,
    uuid: String,
}

pub async fn save_data(
    State(pool): State<PgPool>,
    Form(form): Form<SaveForm>,
) -> Result<String, AppError> {
    let user_id = form.accountID;
    let gjp2 = &form.gjp2;
    let data = &form.saveData;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    User::save_data(&pool, user_id, data).await?;
    Ok("1".to_string())
}
