use axum::{Form, extract::State};
use serde::Serialize;
use serde_deserialize_duplicates::DeserializeFirstDuplicate;
use sqlx::PgPool;

use crate::{
    AppError,
    models::{Level, level::SearchParams},
    util::salt_and_sha1,
};

#[derive(Serialize, DeserializeFirstDuplicate, Debug)]
pub struct SearchForm {
    accountID: i32,
    binaryVersion: i16,
    #[serde(default)]
    coins: i16,
    #[serde(default)]
    completedLevels: String,
    #[serde(default)]
    customSong: i32,
    #[serde(default)]
    demonFilter: i16,
    #[serde(default)]
    diff: String,
    #[serde(default)]
    epic: i16,
    #[serde(default)]
    featured: i16,
    #[serde(default)]
    followed: String,
    gameVersion: i16,
    #[serde(default)]
    gauntlet: i32,
    gjp2: String,
    #[serde(default)]
    legendary: i16,
    #[serde(default)]
    len: String,
    #[serde(default)]
    local: i16,
    #[serde(default)]
    mythic: i16,
    #[serde(default)]
    noStar: i16,
    #[serde(default)]
    onlyCompleted: i16,
    #[serde(default)]
    original: i16,
    #[serde(default)]
    page: i32,
    secret: String,
    #[serde(default)]
    song: i32,
    #[serde(default)]
    star: i16,
    #[serde(default)]
    str: String,
    #[serde(default)]
    total: i32,
    #[serde(default)]
    twoPlayer: i16,
    udid: String,
    #[serde(default)]
    uncompleted: u8,
    uuid: String,
    #[serde(rename = "type")]
    #[serde(default)]
    list_type: i16,
}

pub async fn search_levels(
    State(pool): State<PgPool>,
    Form(form): Form<SearchForm>,
) -> Result<String, AppError> {
    let page = form.page;

    let params = SearchParams { search: form.str };

    let levels: Vec<Level> = Level::search(&pool, params).await?;

    if levels.is_empty() {
        return Ok("-1".to_string());
    }

    let offset = page * 10;
    let count = levels.len();
    let page_info = format!("{count}:{offset}:10");

    let song_string = String::new();

    let response = [
        level_string(&levels),
        creator_string(&levels),
        song_string,
        page_info,
        generate_hash(&levels),
    ];

    Ok(response.join("#"))
}

fn level_string(levels: &Vec<Level>) -> String {
    let mut level_string = String::new();

    for level in levels {
        let temp = Level::to_gd(level);
        level_string.push_str(&temp);
        level_string.push('|');
    }

    level_string.pop();
    level_string
}

fn creator_string(levels: &Vec<Level>) -> String {
    let mut creator_string = String::new();

    for level in levels {
        let creator_id = level.user_id;
        let creator_name = &level.username;

        let temp = format!("{creator_id}:{creator_name}:{creator_id}");
        creator_string.push_str(&temp);
        creator_string.push('|');
    }

    creator_string.pop();
    creator_string
}

pub fn generate_hash(levels: &Vec<Level>) -> String {
    let mut hash = String::new();

    for level in levels {
        let level_id = level.id.to_string();
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
