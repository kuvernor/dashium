use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::AppError;
use crate::models::Post;
use crate::util::verify_gjp2;

#[derive(Serialize, Deserialize, Debug)]
pub struct deleteGJAccComment20 {
    accountID: i32,
    gjp2: String,
    commentID: i32,
    gameVersion: i16,
    binaryVersion: i16,
    secret: String,
    uuid: String,
    udid: String,
}

pub async fn deleteGJAccComment20(
    State(pool): State<PgPool>,
    Form(form): Form<deleteGJAccComment20>,
) -> Result<String, AppError> {
    let user_id = form.accountID;
    let post_id = form.commentID;
    let gjp2 = &form.gjp2;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    Post::delete(&pool, post_id, user_id).await?;
    Ok("1".to_string())
}
