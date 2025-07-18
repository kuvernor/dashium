use anyhow::Result;
use chrono::{DateTime, Utc};
use chrono_humanize::HumanTime;
use serde::Serialize;
use sqlx::{PgPool, prelude::FromRow};

use crate::GDResponse;

#[derive(Debug, FromRow, Serialize)]
#[allow(dead_code)]
pub struct Level {
    pub id: i32,
    pub level_name: String,
    pub description: String,
    pub username: String,
    pub user_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub rated_at: Option<DateTime<Utc>>,

    pub extra_string: String,
    pub level_info: String,
    pub password: String,

    pub version: i32,
    pub length: i16,
    pub official_song: i16,
    pub original: i16,
    pub unlisted: i16,
    pub song_id: i32,
    pub song_ids: String,
    pub sfx_ids: String,
    pub game_version: i16,
    pub binary_version: i16,
    pub auto: bool,
    pub ldm: bool,

    pub likes: i32,
    pub dislikes: i32,
    pub downloads: i32,
    pub objects: i32,
    pub coins: i16,
    pub requested_stars: i16,
    pub legendary: bool,
    pub gauntlet: bool,
    pub two_player: bool,
    pub mythic: bool,
    pub rated: bool,
    pub featured: bool,
    pub difficulty: i16,
    pub demon_difficulty: i16,
    pub demon: bool,
    pub stars: i16,
    pub feature_score: i32,
    pub verified_coins: bool,
    pub wt: i32,
    pub wt2: i32,
    pub daily_number: i16,
    pub epic: bool,
    pub verification_time: i32,
}

impl GDResponse for Level {
    fn to_gd(&self) -> String {
        let auto = if self.auto { 1 } else { 0 };
        let gauntlet = if self.gauntlet { 1 } else { 0 };
        let verified_coins = if self.verified_coins { 1 } else { 0 };
        let ldm = if self.ldm { 1 } else { 0 };
        let rated = if self.rated { 1 } else { 0 };
        let epic = if self.epic { 1 } else { 0 };
        let demon = if self.demon { 1 } else { 0 };
        let two_player = if self.two_player { 1 } else { 0 };

        let response = vec![
            format!("1:{}", self.id),
            format!("2:{}", self.level_name),
            format!("3:{}", self.description),
            format!("5:{}", self.version),
            format!("6:{}", self.user_id),
            format!("8:{rated}"),
            format!("9:{}", self.difficulty),
            format!("10:{}", self.downloads),
            format!("12:{}", self.official_song),
            format!("13:{}", self.game_version),
            format!("14:{}", self.likes),
            format!("15:{}", self.length),
            format!("16:{}", self.dislikes),
            format!("17:{demon}"),
            format!("18:{}", self.stars),
            format!("19:{}", self.feature_score),
            format!("25:{auto}"),
            format!("27:{}", self.password),
            format!("28:{}", HumanTime::from(self.created_at)).replace(" ago", ""),
            format!("29:{}", HumanTime::from(self.updated_at)).replace(" ago", ""),
            format!("30:{}", self.original),
            format!("31:{two_player}"),
            format!("35:{}", self.song_id),
            format!("36:{}", self.extra_string),
            format!("37:{}", self.coins),
            format!("38:{verified_coins}"),
            format!("39:{}", self.requested_stars),
            format!("40:{ldm}"),
            format!("42:{epic}"),
            format!("43:{}", self.demon_difficulty),
            format!("44:{gauntlet}"),
            format!("45:{}", self.objects),
            format!("46:{}", self.wt),
            format!("47:{}", self.wt2),
            format!("52:{}", self.song_ids),
            format!("53:{}", self.sfx_ids),
            format!("57:{}", self.verification_time),
        ];

        response.join(":")
    }
}

impl Level {
    pub async fn get(pool: &PgPool, level_id: i32) -> Result<Self> {
        let level = sqlx::query_as!(Self, "SELECT * FROM levels WHERE id = $1", level_id)
            .fetch_one(pool)
            .await?;

        Ok(level)
    }

    pub async fn get_by_name(pool: &PgPool, search: &str) -> Result<Vec<Self>> {
        let levels = sqlx::query_as!(
            Level,
            "SELECT * FROM levels WHERE level_name ILIKE '%' || $1 || '%'",
            search
        )
        .fetch_all(pool)
        .await?;

        Ok(levels)
    }

    pub async fn update_downloads(pool: &PgPool, level_id: i32) -> Result<()> {
        sqlx::query!(
            "UPDATE levels SET downloads = downloads + 1 WHERE id = $1",
            level_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn delete(pool: &PgPool, user_id: i32, level_id: i32) -> Result<()> {
        sqlx::query!(
            "DELETE FROM levels WHERE user_id = $1 AND id = $2",
            user_id,
            level_id,
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
