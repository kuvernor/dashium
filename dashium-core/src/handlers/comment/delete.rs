use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{AppError, models::Comment, util::verify_gjp2};

#[derive(Deserialize, Serialize, Debug)]
pub struct deleteGJComment20 {
    accountID: i32,
    gjp2: String,
    commentID: i32,
    levelID: i32,
    secret: String,
}

pub async fn deleteGJComment20(
    State(pool): State<PgPool>,
    Form(form): Form<deleteGJComment20>,
) -> Result<String, AppError> {
    let user_id = form.accountID;
    let gjp2 = &form.gjp2;
    let comment_id = form.commentID;
    let level_id = form.levelID;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    Comment::delete(&pool, user_id, level_id, comment_id).await?;
    Ok("1".to_string())
}
