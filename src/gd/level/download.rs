use crate::GDResponse;
use crate::{AppError, models::Level, util::salt_and_sha1};
use axum::{Form, extract::State};
use chrono::Utc;
use serde::Serialize;
use serde_deserialize_duplicates::DeserializeFirstDuplicate;
use sqlx::PgPool;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[derive(DeserializeFirstDuplicate, Serialize, Debug)]
pub struct DownloadForm {
    accountID: i32,
    gjp2: String,
    levelID: i32,
    secret: String,
}

pub async fn download_level(
    State(pool): State<PgPool>,
    Form(form): Form<DownloadForm>,
) -> Result<String, AppError> {
    let level_id = form.levelID;
    let mut daily = false;
    let mut daily_id = 0;
    let mut actual_level_id = 0;
    let now = Utc::now();

    let level = match level_id {
        -1 => {
            daily = true;
            let row = sqlx::query!(
                "SELECT id, level_id FROM daily_levels WHERE created_at < $1 ORDER BY created_at DESC", now)
                .fetch_one(&pool).await?;

            daily_id = row.id;
            actual_level_id = row.level_id;

            sqlx::query_as!(Level, "SELECT * FROM levels WHERE id = $1", row.level_id)
                .fetch_one(&pool)
                .await?
        }
        -2 => {
            daily = true;
            let row = sqlx::query!(
                "SELECT id + 100000 AS daily_id, level_id FROM weekly_demons WHERE created_at < $1 ORDER BY created_at DESC", now)
                .fetch_one(&pool).await?;

            daily_id = row.daily_id.unwrap_or(0);
            actual_level_id = row.level_id;

            sqlx::query_as!(Level, "SELECT * FROM levels WHERE id = $1", row.level_id)
                .fetch_one(&pool)
                .await?
        }
        -3 => {
            daily = true;
            let row = sqlx::query!(
                "SELECT id + 200000 AS daily_id, level_id FROM event_levels WHERE created_at < $1 ORDER BY created_at DESC", now)
                .fetch_one(&pool).await?;

            daily_id = row.daily_id.unwrap_or(0);
            actual_level_id = row.level_id;

            sqlx::query_as!(Level, "SELECT * FROM levels WHERE id = $1", row.level_id)
                .fetch_one(&pool)
                .await?
        }
        _ => Level::get(&pool, level_id).await?,
    };

    let path = if daily {
        format!("./data/levels/{actual_level_id}.level")
    } else {
        format!("./data/levels/{level_id}.level")
    };

    let path = Path::new(&path);
    let mut file = File::open(path).await?;
    let mut level_data = vec![];
    file.read_to_end(&mut level_data).await?;
    let level_data = String::from_utf8(level_data)?;

    let hash1 = generate_hash1(&level_data);
    let hash2 = generate_hash2(&level, daily_id);

    let response = if daily {
        let user_string = format!("{}:{}:{}", level.user_id, level.username, level.user_id);
        vec![
            format!("41:{daily_id}:"),
            format!("4:{level_data}:"),
            level.to_gd(),
            format!("#{hash1}"),
            format!("#{hash2}"),
            format!("#{user_string}"),
        ]
    } else {
        vec![
            format!("4:{}:", level_data),
            level.to_gd(),
            format!("#{hash1}"),
            format!("#{hash2}"),
        ]
    };

    Ok(response.join(""))
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

pub fn generate_hash2(level: &Level, daily_id: i32) -> String {
    let user_id = level.user_id;
    let stars = level.stars;
    let demon = level.demon;

    let level_id = level.id;
    let verified_coins = level.verified_coins;
    let feature_score = level.feature_score;
    let password = &level.password;

    let hash = format!(
        "{user_id},{stars},{demon},{level_id},{verified_coins},{feature_score},{password},{daily_id}"
    );

    salt_and_sha1(&hash, "xI25fpAapCQg")
}
