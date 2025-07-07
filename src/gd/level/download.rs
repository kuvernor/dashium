use axum::{Form, extract::State};
use serde::Serialize;
use serde_deserialize_duplicates::DeserializeFirstDuplicate;
use sqlx::PgPool;

use crate::{AppError, models::Level, util::salt_and_sha1};

#[derive(DeserializeFirstDuplicate, Serialize, Debug)]
pub struct DownloadForm {
    accountID: i32,
    binaryVersion: i16,
    gameVersion: i16,
    gjp2: String,
    levelID: i32,
    secret: String,
    udid: String,
    uuid: String,
    #[serde(default)]
    chk: String,
    #[serde(default)]
    inc: String,
    #[serde(default)]
    rs: String,
}

pub async fn download_level(
    State(pool): State<PgPool>,
    Form(form): Form<DownloadForm>,
) -> Result<String, AppError> {
    let level_id = form.levelID;

    let level = &Level::get(&pool, level_id).await?;

    Level::update_downloads(&pool, level_id).await?;

    let hash1 = generate_hash1(&level.level_string);
    let hash2 = generate_hash2(level);

    let response = [Level::to_gd(level, true), hash1, hash2];

    Ok(response.join("#"))
}

pub fn generate_hash1(level_string: &str) -> String {
    let salt = "xI25fpAapCQg";

    if level_string.len() < 41 {
        return salt_and_sha1(level_string, salt);
    }

    let mut hash_chars: Vec<char> = "????????????????????????????????????????xI25fpAapCQg"
        .chars()
        .collect();
    let m = level_string.len() / 40;

    for i in (0..40).rev() {
        if let Some(c) = level_string.chars().nth(i * m) {
            hash_chars[i] = c;
        }
    }

    let hash: String = hash_chars.into_iter().collect();

    salt_and_sha1(&hash, "")
}

pub fn generate_hash2(level: &Level) -> String {
    let user_id = level.user_id;
    let stars = level.stars;
    let is_demon = level.is_demon;
    let level_id = level.level_id;
    let verified_coins = level.verified_coins;
    let feature_score = level.feature_score;
    let password = &level.password;
    let daily_number = level.daily_number;

    let hash = format!(
        "{user_id},{stars},{is_demon},{level_id},{verified_coins},{feature_score},{password},{daily_number}"
    );

    salt_and_sha1(&hash, "xI25fpAapCQg")
}
