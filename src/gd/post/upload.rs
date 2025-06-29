use axum::{Form, extract::State};
use serde::Serialize;
use serde_deserialize_duplicates::DeserializeFirstDuplicate;
use sqlx::PgPool;
use tracing::info;

use crate::AppError;
use crate::models::Post;
use crate::util::verify_gjp2;

#[derive(Serialize, DeserializeFirstDuplicate, Debug)]
pub struct UploadForm {
    #[serde(rename = "accountID")]
    user_id: i32,
    gjp2: String,

    #[serde(rename = "comment")]
    body: String,

    #[serde(rename = "userName")]
    username: String,

    #[serde(rename = "gameVersion")]
    game_version: i16,

    #[serde(rename = "binaryVersion")]
    binary_version: i16,

    #[serde(default)]
    gdw: u8,
    secret: String,
    uuid: String,
    udid: String,
    chk: String,

    #[serde(rename = "cType")]
    c_type: String,
}

pub async fn upload_post(
    State(pool): State<PgPool>,
    Form(form): Form<UploadForm>,
) -> Result<String, AppError> {
    let user_id = form.user_id;
    let gjp2 = &form.gjp2;
    let body = &form.body;
    let username = &form.username;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    let response = Post::upload(&pool, body, user_id).await?;

    info!("{} uploaded a post!", username);

    Ok(response)
}
