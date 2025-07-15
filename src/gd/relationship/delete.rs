use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{AppError, models::FriendRequest, util::verify_gjp2};

#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteForm {
    accountID: i32,
    gjp2: String,
    #[serde(default)]
    accounts: String,
    targetAccountID: i32,
    isSender: i16,
    secret: String,
}

pub async fn delete_friend_request(
    State(pool): State<PgPool>,
    Form(form): Form<DeleteForm>,
) -> Result<String, AppError> {
    let user_id = form.accountID;
    let gjp2 = &form.gjp2;
    let target_id = form.targetAccountID;
    let target_ids = &form.accounts;
    let is_sender = form.isSender;

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
