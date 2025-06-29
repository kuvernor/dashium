use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::AppError;
use crate::models::User;

#[derive(Serialize, Deserialize, Debug)]
pub struct InfoForm {
    #[serde(rename = "accountID")]
    user_id: i32,

    #[serde(rename = "targetAccountID")]
    target_id: i32,

    gjp2: String,

    #[serde(rename = "gameVersion")]
    game_version: i16,

    #[serde(rename = "binaryVersion")]
    binary_version: i16,

    #[serde(default)]
    gdw: u8,

    secret: String,
}

pub async fn info(
    State(pool): State<PgPool>,
    Form(form): Form<InfoForm>,
) -> Result<String, AppError> {
    let user_id = form.user_id;

    let user = User::get_user(&pool, user_id).await?;
    let response = User::to_gd(user);

    Ok(response)
}
