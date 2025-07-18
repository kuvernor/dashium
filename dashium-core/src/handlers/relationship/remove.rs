use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{AppError, models::Friendship, util::verify_gjp2};

#[derive(Deserialize, Serialize, Debug)]
pub struct removeGJFriend20 {
    accountID: i32,
    gjp2: String,
    targetAccountID: i32,
    secret: String,
}

pub async fn removeGJFriend20(
    State(pool): State<PgPool>,
    Form(form): Form<removeGJFriend20>,
) -> Result<String, AppError> {
    let user_id = form.accountID;
    let gjp2 = &form.gjp2;
    let target_id = form.targetAccountID;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    Friendship::delete(&pool, user_id, target_id).await?;

    Ok("1".to_string())
}
