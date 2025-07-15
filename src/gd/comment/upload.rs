use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    AppError,
    models::Comment,
    util::{base64_decode, verify_gjp2},
};

#[derive(Deserialize, Serialize, Debug)]
pub struct UploadForm {
    accountID: i32,
    gjp2: String,
    gameVersion: i16,
    binaryVersion: i16,
    secret: String,
    udid: String,
    uuid: String,
    chk: String,
    comment: String,
    levelID: i32,
    #[serde(default)]
    percent: i16,
    userName: String,
}

pub async fn upload_comment(
    State(pool): State<PgPool>,
    Form(form): Form<UploadForm>,
) -> Result<String, AppError> {
    let user_id = form.accountID;
    let gjp2 = &form.gjp2;
    let level_id = form.levelID;
    let username = &form.userName;
    let comment = &String::from_utf8(base64_decode(&form.comment)?)?;
    let percent = form.percent;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    match Comment::upload(&pool, user_id, username, level_id, comment, percent).await {
        Ok(response) => Ok(response.to_string()),
        Err(_) => Ok("-1".to_string()),
    }
}
