use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    AppError,
    models::Comment,
    util::{base64_decode, verify_gjp2},
};

#[derive(Deserialize, Serialize, Debug)]
pub struct uploadGJComment21 {
    accountID: i32,
    gjp2: String,
    secret: String,
    comment: String,
    levelID: i32,
    #[serde(default)]
    percent: i16,
    userName: String,
}

pub async fn uploadGJComment21(
    State(pool): State<PgPool>,
    Form(form): Form<uploadGJComment21>,
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

    let comment_id = Comment::upload(&pool, user_id, username, level_id, comment, percent).await?;
    Ok(comment_id.to_string())
}
