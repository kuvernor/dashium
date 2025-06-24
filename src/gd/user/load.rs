use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{AppError, models::User, util::verify_gjp2};

#[derive(Deserialize, Serialize, Debug)]
pub struct LoadForm {
    #[serde(rename = "accountID")]
    user_id: i32,
    gjp2: String,
    #[serde(rename = "gameVersion")]
    game_version: i16,
    #[serde(rename = "binaryVersion")]
    binary_version: i16,
    #[serde(rename = "saveData")]
    secret: Option<String>,
    gdw: Option<String>,
    udid: Option<String>,
    uuid: Option<String>,
}

pub async fn load_data(
    State(pool): State<PgPool>,
    Form(form): Form<LoadForm>,
) -> Result<String, AppError> {
    let user_id = form.user_id;
    let gjp2 = &form.gjp2;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok(String::from("-1"));
    }

    match User::load_data(&pool, user_id).await {
        Ok(save_data) => Ok(format!("{save_data};21;30;a;a")),
        Err(_) => return Ok("-1".to_string()),
    }
}
