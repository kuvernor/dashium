use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::{
    AppError,
    util::{base64_decode, int_to_bool, verify_gjp2},
};

#[derive(Deserialize, Serialize, Debug)]
pub struct uploadGJLevel21 {
    accountID: i32,
    audioTrack: i16,
    binaryVersion: i16,
    coins: i16,
    gameVersion: i16,
    gjp2: String,
    #[serde(deserialize_with = "int_to_bool")]
    ldm: bool,
    levelDesc: String,
    levelID: i32,
    levelInfo: String,
    levelLength: i16,
    levelName: String,
    levelString: String,
    levelVersion: i32,
    objects: i32,
    original: i16,
    password: String,
    requestedStars: i16,
    secret: String,
    songID: i32,
    #[serde(deserialize_with = "int_to_bool")]
    twoPlayer: bool,
    unlisted: i16,
    userName: String,
    wt: i32,
    wt2: i32,

    #[serde(default)]
    extraString: String,
}

pub async fn uploadGJLevel21(
    State(pool): State<PgPool>,
    Form(form): Form<uploadGJLevel21>,
) -> Result<String, AppError> {
    let user_id = form.accountID;
    let gjp2 = &form.gjp2;
    let official_song = form.audioTrack;
    let binary_version = form.binaryVersion;
    let coins = form.coins;
    let game_version = form.gameVersion;
    let ldm = form.ldm;
    let level_info = &form.levelInfo;
    let length = form.levelLength;
    let level_name = &form.levelName;
    let level_string = &form.levelString;
    let version = form.levelVersion;
    let objects = form.objects;
    let original = form.original;
    let password = &form.password;
    let requested_stars = form.requestedStars;
    let song_id = form.songID;
    let two_player = form.twoPlayer;
    let unlisted = form.unlisted;
    let username = &form.userName;
    let wt = form.wt;
    let wt2 = form.wt2;
    let extra_string = &form.extraString;

    let description = String::from_utf8(base64_decode(&form.levelDesc)?)?;

    if !verify_gjp2(&pool, user_id, gjp2).await? {
        return Ok("-1".to_string());
    }

    let level_id: i32 = sqlx::query_scalar!(
        r#"
        INSERT INTO levels (
            user_id,
            official_song,
            binary_version,
            coins,
            game_version,
            ldm,
            description,
            level_info,
            length,
            level_name,
            version,
            objects,
            original,
            password,
            requested_stars,
            song_id,
            two_player,
            unlisted,
            username,
            wt,
            wt2,
            extra_string
        )
        VALUES (
            $1,
            $2,
            $3,
            $4,
            $5,
            $6,
            $7,
            $8,
            $9,
            $10,
            $11,
            $12,
            $13,
            $14,
            $15,
            $16,
            $17,
            $18,
            $19,
            $20,
            $21,
            $22
        )
        RETURNING id
        "#,
        user_id,
        official_song,
        binary_version,
        coins,
        game_version,
        ldm,
        description,
        level_info,
        length,
        level_name,
        version,
        objects,
        original,
        password,
        requested_stars,
        song_id,
        two_player,
        unlisted,
        username,
        wt,
        wt2,
        extra_string
    )
    .fetch_one(&pool)
    .await?;

    let path = format!("./data/levels/{level_id}.level");
    let path = Path::new(&path);
    let mut file = File::create(path).await?;
    file.write_all(level_string.as_bytes()).await?;
    file.flush().await?;

    Ok(level_id.to_string())
}
