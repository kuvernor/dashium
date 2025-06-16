use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{AppError, models::User};

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

    let verified = User::verify_password(&pool, user_id, gjp2).await?;
    if !verified {
        return Ok(String::from("-1"));
    }

    let save_data = sqlx::query_scalar!("SELECT save_data FROM users WHERE id = $1", user_id)
        .fetch_one(&pool)
        .await?;

    Ok(format!("{save_data};21;30;a;a"))
}
