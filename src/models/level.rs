use anyhow::Result;
use chrono::{DateTime, Utc};
use chrono_humanize::HumanTime;
use serde::Serialize;
use sqlx::{PgPool, Postgres, QueryBuilder, prelude::FromRow};

use crate::util::is_numeric;

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

    pub extra_string: String,
    pub level_info: String,
    pub password: String,

    pub version: i32,
    pub length: i16,
    pub official_song: i16,
    pub original: i16,
    pub unlisted: i16,
    pub is_two_player: i16,
    pub song_id: i32,
    pub song_ids: String,
    pub sfx_ids: String,
    pub game_version: i16,
    pub binary_version: i16,
    pub is_auto: i16,
    pub is_ldm: i16,

    pub likes: i32,
    pub dislikes: i32,
    pub downloads: i32,
    pub objects: i32,
    pub coins: i16,
    pub requested_stars: i16,

    pub is_rated: i16,
    pub difficulty: i16,
    pub demon_difficulty: i16,
    pub is_demon: i16,
    pub stars: i16,
    pub feature_score: i32,
    pub verified_coins: i16,
    pub wt: i32,
    pub wt2: i32,
    pub daily_number: i16,
    pub epic: i16,
    pub is_gauntlet: i16,
    pub verification_time: i32,
}

pub struct SearchParams {
    pub search: String,
}

impl Level {
    pub fn to_gd(level: &Self) -> String {
        let response = vec![
            format!("1:{}", level.id),
            format!("2:{}", level.level_name),
            format!("3:{}", level.description),
            format!("5:{}", level.version),
            format!("6:{}", level.user_id),
            format!("8:{}", level.is_rated),
            format!("9:{}", level.difficulty),
            format!("10:{}", level.downloads),
            format!("12:{}", level.official_song),
            format!("13:{}", level.game_version),
            format!("14:{}", level.likes),
            format!("15:{}", level.length),
            format!("16:{}", level.dislikes),
            format!("17:{}", level.is_demon),
            format!("18:{}", level.stars),
            format!("19:{}", level.feature_score),
            format!("25:{}", level.is_auto),
            format!("27:{}", level.password),
            format!("28:{}", HumanTime::from(level.created_at)).replace(" ago", ""),
            format!("29:{}", HumanTime::from(level.updated_at)).replace(" ago", ""),
            format!("30:{}", level.original),
            format!("31:{}", level.is_two_player),
            format!("35:{}", level.song_id),
            format!("36:{}", level.extra_string),
            format!("37:{}", level.coins),
            format!("38:{}", level.verified_coins),
            format!("39:{}", level.requested_stars),
            format!("40:{}", level.is_ldm),
            format!("41:{}", level.daily_number),
            format!("42:{}", level.epic),
            format!("43:{}", level.demon_difficulty),
            format!("44:{}", level.is_gauntlet),
            format!("45:{}", level.objects),
            format!("46:{}", level.wt),
            format!("47:{}", level.wt2),
            format!("52:{}", level.song_ids),
            format!("53:{}", level.sfx_ids),
            format!("57:{}", level.verification_time),
        ];

        response.join(":")
    }

    pub async fn get(pool: &PgPool, level_id: i32) -> Result<Self> {
        let level = sqlx::query_as!(Self, "SELECT * FROM levels WHERE id = $1", level_id)
            .fetch_one(pool)
            .await?;

        Ok(level)
    }

    pub async fn search(pool: &PgPool, params: SearchParams) -> Result<Vec<Self>> {
        let mut query: QueryBuilder<Postgres> = QueryBuilder::new("SELECT * FROM levels");

        if is_numeric(&params.search) {
            query.push(" WHERE id = ");
            query.push_bind(params.search);
        } else {
            query.push(" WHERE level_name ILIKE '%' || ");
            query.push_bind(params.search);
            query.push(" || '%'");
        }

        let levels = query.build_query_as().fetch_all(pool).await?;

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
