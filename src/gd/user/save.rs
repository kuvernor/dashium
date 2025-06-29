use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{AppError, models::User, util::verify_gjp2};

#[derive(Deserialize, Serialize, Debug)]
pub struct SaveForm {
    #[serde(rename = "accountID")]
    user_id: i32,
    gjp2: String,

    #[serde(rename = "gameVersion")]
    game_version: i16,

    #[serde(rename = "binaryVersion")]
    binary_version: i16,

    #[serde(rename = "saveData")]
    save_data: String,

    secret: String,
    udid: String,
    uuid: String,
}

pub async fn save_data(
    State(pool): State<PgPool>,
    Form(form): Form<SaveForm>,
) -> Result<String, AppError> {
    let user_id = form.user_id;
    let gjp2 = &form.gjp2;
    let data = &form.save_data;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    match User::save_data(&pool, user_id, data).await {
        Ok(_) => return Ok("1".to_string()),
        Err(_) => return Ok("-1".to_string()),
    }
}
