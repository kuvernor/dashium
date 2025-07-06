use axum::{Form, extract::State};
use serde::Serialize;
use serde_deserialize_duplicates::DeserializeFirstDuplicate;
use sqlx::PgPool;

use crate::AppError;
use crate::models::Post;
use crate::util::verify_gjp2;

#[derive(Serialize, DeserializeFirstDuplicate, Debug)]
pub struct UploadForm {
    accountID: i32,
    binaryVersion: i16,
    chk: String,
    comment: String,
    cType: String,
    gameVersion: i16,
    gjp2: String,
    secret: String,
    udid: String,
    userName: String,
    uuid: String,
}

pub async fn upload_post(
    State(pool): State<PgPool>,
    Form(form): Form<UploadForm>,
) -> Result<String, AppError> {
    let user_id = form.accountID;
    let gjp2 = &form.gjp2;
    let body = &form.comment;
    let username = &form.userName;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    let response = Post::upload(&pool, body, user_id, username).await?;

    Ok(response)
}
