use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{AppError, models::Friendship, util::verify_gjp2};

#[derive(Deserialize, Serialize, Debug)]
pub struct RemoveForm {
    #[serde(rename = "accountID")]
    user_id: i32,
    gjp2: String,

    #[serde(rename = "targetAccountID")]
    target_id: i32,

    #[serde(rename = "gameVersion")]
    game_version: i16,

    #[serde(rename = "binaryVersion")]
    binary_version: i16,

    secret: String,
    udid: String,
    uuid: String,
}

pub async fn remove_friend(
    State(pool): State<PgPool>,
    Form(form): Form<RemoveForm>,
) -> Result<String, AppError> {
    let user_id = form.user_id;
    let gjp2 = &form.gjp2;
    let target_id = form.target_id;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    Friendship::delete(&pool, user_id, target_id).await?;

    Ok("1".to_string())
}
