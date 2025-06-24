use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::info;

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
    game_version: Option<i16>,
    #[serde(rename = "binaryVersion")]
    binary_version: Option<i16>,
    gdw: Option<u8>,
    secret: String,
}

pub async fn info(
    State(pool): State<PgPool>,
    Form(form): Form<InfoForm>,
) -> Result<String, AppError> {
    let user = User::to_gd(&pool, form.target_id).await?;

    info!(
        "ID {} requested info for ID {}.",
        form.user_id, form.target_id
    );

    Ok(user)
}
