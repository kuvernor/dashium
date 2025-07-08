use axum::{Form, extract::State};
use serde::Serialize;
use serde_deserialize_duplicates::DeserializeFirstDuplicate;
use sqlx::PgPool;

use crate::{AppError, models::Level, util::salt_and_sha1};

#[derive(Serialize, DeserializeFirstDuplicate, Debug)]
pub struct SearchForm {
    accountID: i32,
    binaryVersion: i16,
    coins: i16,
    #[serde(default)]
    completedLevels: String,
    #[serde(default)]
    customSong: i32,
    #[serde(default)]
    demonFilter: i16,
    diff: String,
    #[serde(default)]
    epic: i16,
    featured: i16,
    #[serde(default)]
    followed: String,
    gameVersion: i16,
    #[serde(default)]
    gauntlet: i32,
    gjp2: String,
    #[serde(default)]
    legendary: i16,
    len: String,
    #[serde(default)]
    local: i16,
    #[serde(default)]
    mythic: i16,
    #[serde(default)]
    noStar: i16,
    onlyCompleted: i16,
    original: i16,
    page: i32,
    secret: String,
    #[serde(default)]
    song: i32,
    #[serde(default)]
    star: i16,
    str: String,
    total: i32,
    twoPlayer: i16,
    udid: String,
    uncompleted: u8,
    uuid: String,
    #[serde(rename = "type")]
    list_type: i16,
}

pub async fn search_levels(
    State(pool): State<PgPool>,
    Form(form): Form<SearchForm>,
) -> Result<String, AppError> {
    let page = form.page;
    let search = &form.str;

    let levels: Vec<Level> = Level::get_all(&pool, search, search).await?;

    if levels.is_empty() {
        return Ok("-1".to_string());
    }

    let offset = page * 10;
    let count = levels.len();
    let page_info = format!("{count}:{offset}:10");

    let hash = generate_hash(&levels);

    let mut level_response = String::new();

    for level in &levels {
        let temp = Level::to_gd(level, false);
        level_response.push_str(&temp);
        level_response.push('|');
    }

    level_response.pop();

    let mut creator_response = String::new();

    for level in &levels {
        let creator_id = level.user_id;
        let creator_name = &level.username;

        let temp = format!("{creator_id}:{creator_name}:{creator_id}");
        creator_response.push_str(&temp);
        creator_response.push('|');
    }

    creator_response.pop();

    let song_response = String::new();

    let response = [
        level_response,
        creator_response,
        song_response,
        page_info,
        hash,
    ];
    let response = response.join("#");

    Ok(response)
}

pub fn generate_hash(levels: &Vec<Level>) -> String {
    let mut hash = String::new();

    for level in levels {
        let level_id = level.level_id.to_string();
        let first = level_id.chars().next().unwrap_or('0');
        let last = level_id.chars().next_back().unwrap_or('0');
        let stars = level.stars.to_string();
        let verified_coins = level.verified_coins.to_string();
        hash.push(first);
        hash.push(last);
        hash.push_str(&stars);
        hash.push_str(&verified_coins);
    }

    salt_and_sha1(&hash, "xI25fpAapCQg")
}
