use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{FromRow, PgPool};

use crate::{
    GDResponse,
    util::{hash_gjp2, salt_and_sha1},
};

#[derive(FromRow, Serialize)]
pub struct User {
    pub id: i32,

    #[serde(skip)]
    #[sqlx(skip)]
    pub hash: String,

    #[serde(skip)]
    #[sqlx(skip)]
    pub email: String,

    #[serde(skip)]
    #[sqlx(skip)]
    pub save_data: String,

    pub username: String,
    pub stars: i32,
    pub demons: i32,
    pub creator_points: i32,
    pub diamonds: i32,
    pub moons: i32,
    pub coins: i32,
    pub user_coins: i32,
    pub message_setting: i16,
    pub friend_setting: i16,
    pub comment_setting: i16,
    pub youtube: String,
    pub twitter: String,
    pub twitch: String,
    pub cube: i16,
    pub ship: i16,
    pub ball: i16,
    pub ufo: i16,
    pub wave: i16,
    pub robot: i16,
    pub glow: i16,
    pub spider: i16,
    pub explosion: i16,
    pub swing: i16,
    pub jetpack: i16,
    pub color1: i16,
    pub color2: i16,
    pub color3: i16,
    pub icon_type: i16,
    pub display_icon: i16,
    pub activated: bool,
    pub rank: Option<i64>,
    pub mod_level: i16,
    pub demon_info: String,
    pub level_info: String,
    pub platformer_info: String,
    pub created_at: DateTime<Utc>,
}

impl GDResponse for User {
    fn to_gd(&self) -> String {
        let activated = if self.activated { 1 } else { 0 };

        let response = [
            format!("1:{}", self.username),
            format!("2:{}", self.id),
            format!("3:{}", self.stars),
            format!("4:{}", self.demons),
            format!("8:{}", self.creator_points),
            format!("9:{}", self.display_icon),
            format!("10:{}", self.color1),
            format!("11:{}", self.color2),
            format!("13:{}", self.coins),
            format!("14:{}", self.icon_type),
            format!("15:{}", self.glow),
            format!("16:{}", self.id),
            format!("17:{}", self.coins),
            format!("18:{}", self.message_setting),
            format!("19:{}", self.friend_setting),
            format!("20:{}", self.youtube),
            format!("21:{}", self.cube),
            format!("22:{}", self.ship),
            format!("23:{}", self.ball),
            format!("24:{}", self.ufo),
            format!("25:{}", self.wave),
            format!("26:{}", self.robot),
            format!("28:{}", self.glow),
            format!("29:{activated}"),
            format!("30:{}", self.rank.unwrap_or(0)),
            format!("43:{}", self.spider),
            format!("44:{}", self.twitter),
            format!("45:{}", self.twitch),
            format!("46:{}", self.diamonds),
            format!("48:{}", self.explosion),
            format!("49:{}", self.mod_level),
            format!("50:{}", self.comment_setting),
            format!("51:{}", self.color3),
            format!("52:{}", self.moons),
            format!("53:{}", self.swing),
            format!("54:{}", self.jetpack),
            format!("55:{}", self.demon_info),
            format!("56:{}", self.level_info),
            format!("57:{}", self.platformer_info),
        ];

        response.join(":")
    }
}

impl User {
    pub async fn get_user(pool: &PgPool, user_id: i32) -> Result<Self> {
        let user = sqlx::query_as("SELECT * FROM user_view WHERE id = $1")
            .bind(user_id)
            .fetch_one(pool)
            .await?;

        Ok(user)
    }

    pub async fn get_by_name(pool: &PgPool, search: &str) -> Result<Vec<Self>> {
        let users = sqlx::query_as(
            "SELECT * FROM user_view WHERE username ILIKE '%' || $1 || '%' LIMIT 100",
        )
        .bind(search)
        .fetch_all(pool)
        .await?;

        Ok(users)
    }

    pub async fn id_from_username(pool: &PgPool, username: &str) -> Result<i32> {
        let user_id = sqlx::query_scalar!("SELECT id FROM users WHERE username ILIKE $1", username)
            .fetch_one(pool)
            .await?;
        Ok(user_id)
    }

    pub async fn username_from_id(pool: &PgPool, user_id: i32) -> Result<String> {
        let username = sqlx::query_scalar!("SELECT username FROM users WHERE id = $1", user_id)
            .fetch_one(pool)
            .await?;

        Ok(username)
    }

    pub async fn create(pool: &PgPool, username: &str, password: &str, email: &str) -> Result<()> {
        let gjp2 = salt_and_sha1(password, "mI29fmAnxgTs");
        let hash = hash_gjp2(&gjp2)?;

        sqlx::query!(
            "INSERT INTO users (username, hash, email) VALUES ($1, $2, $3)",
            username,
            hash,
            email
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn is_username_taken(pool: &PgPool, username: &str) -> Result<bool> {
        let exists: bool = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM users WHERE username ILIKE $1) AS \"exists!\"",
            username
        )
        .fetch_one(pool)
        .await?;

        Ok(exists)
    }

    pub async fn is_email_taken(pool: &PgPool, email: &str) -> Result<bool> {
        let exists: bool = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM users WHERE email ILIKE $1) AS \"exists!\"",
            email
        )
        .fetch_one(pool)
        .await?;

        Ok(exists)
    }

    pub async fn save_data(pool: &PgPool, user_id: i32, data: &str) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE users
            SET save_data = $1
            WHERE id = $2
            "#,
            data,
            user_id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn load_data(pool: &PgPool, user_id: i32) -> Result<String> {
        let data = sqlx::query_scalar!("SELECT save_data FROM users WHERE id = $1", user_id)
            .fetch_one(pool)
            .await?;

        Ok(data)
    }
}
