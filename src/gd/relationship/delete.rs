use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{AppError, models::FriendRequest, util::verify_gjp2};

#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteForm {
    #[serde(rename = "accountID")]
    user_id: i32,

    gjp2: String,

    #[serde(default)]
    #[serde(rename = "accounts")]
    target_ids: String,

    #[serde(rename = "targetAccountID")]
    target_id: i32,

    #[serde(rename = "isSender")]
    is_sender: i16,

    #[serde(rename = "gameVersion")]
    game_version: i16,

    #[serde(rename = "binaryVersion")]
    binary_version: i16,

    secret: String,
    udid: String,
    uuid: String,
}

pub async fn delete_friend_request(
    State(pool): State<PgPool>,
    Form(form): Form<DeleteForm>,
) -> Result<String, AppError> {
    let user_id = form.user_id;
    let gjp2 = &form.gjp2;
    let target_id = form.target_id;
    let target_ids = &form.target_ids;
    let is_sender = form.is_sender;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("1".to_string());
    }

    if target_ids.is_empty() {
        match is_sender {
            0 => FriendRequest::delete(&pool, target_id, user_id, false).await?,
            1 => FriendRequest::delete(&pool, user_id, target_id, true).await?,
            _ => (),
        }
    } else {
        let target_ids: Vec<i32> = target_ids
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        match is_sender {
            0 => {
                for target_id in target_ids {
                    FriendRequest::delete(&pool, target_id, user_id, false).await?;
                }
            }
            1 => {
                for target_id in target_ids {
                    FriendRequest::delete(&pool, user_id, target_id, true).await?;
                }
            }
            _ => (),
        }
    }

    Ok("1".to_string())
}
