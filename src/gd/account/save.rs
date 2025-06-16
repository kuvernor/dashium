use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{AppError, models::User};

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
    secret: Option<String>,
    udid: Option<String>,
    uuid: Option<String>,
}

pub async fn save_data(
    State(pool): State<PgPool>,
    Form(form): Form<SaveForm>,
) -> Result<String, AppError> {
    let user_id = form.user_id;
    let gjp2 = &form.gjp2;
    let save_data = &form.save_data;

    let verified = User::verify_password(&pool, user_id, gjp2).await?;
    if !verified {
        return Ok(String::from("-1"));
    }

    sqlx::query!(
        r#"
        UPDATE users
        SET save_data = $1
        WHERE id = $2
        "#,
        save_data,
        user_id
    )
    .execute(&pool)
    .await?;

    Ok("1".to_string())
}
