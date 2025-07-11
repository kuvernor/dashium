use anyhow::Result;
use serde::Serialize;
use sqlx::{FromRow, PgPool};

use crate::util::{hash_gjp2, salt_and_sha1};

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    id: i32,
    pub username: String,
    stars: i32,
    demons: i32,
    creator_points: i32,
    pub color1: i16,
    pub color2: i16,
    coins: i32,
    pub icon_type: i16,
    display_icon: i16,
    user_coins: i32,
    message_setting: i16,
    friend_setting: i16,
    youtube: String,
    pub icon: i16,
    ship: i16,
    ball: i16,
    ufo: i16,
    wave: i16,
    robot: i16,
    pub glow: i16,
    is_activated: i16,
    rank: i32,
    spider: i16,
    twitter: String,
    twitch: String,
    diamonds: i32,
    explosion: i16,
    pub mod_level: i16,
    comment_setting: i16,
    color3: i16,
    moons: i32,
    swing: i16,
    jetpack: i16,
    demon_info: String,
    level_info: String,
    platformer_info: String,
}

impl User {
    pub fn to_gd(user: User) -> String {
        let response: Vec<String> = vec![
            format!("1:{}", user.username),
            format!("2:{}", user.id),
            format!("3:{}", user.stars),
            format!("4:{}", user.demons),
            format!("8:{}", user.creator_points),
            format!("9:{}", user.display_icon),
            format!("10:{}", user.color1),
            format!("11:{}", user.color2),
            format!("13:{}", user.coins),
            format!("14:{}", user.icon_type),
            format!("15:{}", user.glow),
            format!("16:{}", user.id),
            format!("17:{}", user.user_coins),
            format!("18:{}", user.message_setting),
            format!("19:{}", user.friend_setting),
            format!("20:{}", user.youtube),
            format!("21:{}", user.icon),
            format!("22:{}", user.ship),
            format!("23:{}", user.ball),
            format!("24:{}", user.ufo),
            format!("25:{}", user.wave),
            format!("26:{}", user.robot),
            format!("28:{}", user.glow),
            format!("29:{}", user.is_activated),
            format!("30:{}", user.rank),
            format!("43:{}", user.spider),
            format!("44:{}", user.twitter),
            format!("45:{}", user.twitch),
            format!("46:{}", user.diamonds),
            format!("48:{}", user.explosion),
            format!("49:{}", user.mod_level),
            format!("50:{}", user.comment_setting),
            format!("51:{}", user.color3),
            format!("52:{}", user.moons),
            format!("53:{}", user.swing),
            format!("54:{}", user.jetpack),
            format!("55:{}", user.demon_info),
            format!("56:{}", user.level_info),
            format!("57:{}", user.platformer_info),
        ];
        response.join(":")
    }

    pub async fn get_user(pool: &PgPool, user_id: i32) -> Result<Self> {
        let user = sqlx::query_as!(
            Self,
            r#"
            SELECT  
            id,
            username,
            stars,
            demons,
            creator_points,
            color1,
            color2,
            coins,
            icon_type,
            display_icon,
            user_coins,
            message_setting,
            friend_setting,
            youtube,
            icon,
            ship,
            ball,
            ufo,
            wave,
            robot,
            glow,
            is_activated,
            rank,
            spider,
            twitter,
            twitch,
            diamonds,
            explosion,
            mod_level,
            comment_setting,
            color3,
            moons,
            swing,
            jetpack,
            demon_info,
            level_info,
            platformer_info
            FROM users WHERE id = $1
            "#,
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn get_users(pool: &PgPool, search: &str) -> Result<Vec<Self>> {
        let users: Vec<User> = sqlx::query_as!(
            User,
            r#"
            SELECT  
            id,
            username,
            stars,
            demons,
            creator_points,
            color1,
            color2,
            coins,
            icon_type,
            display_icon,
            user_coins,
            message_setting,
            friend_setting,
            youtube,
            icon,
            ship,
            ball,
            ufo,
            wave,
            robot,
            glow,
            is_activated,
            rank,
            spider,
            twitter,
            twitch,
            diamonds,
            explosion,
            mod_level,
            comment_setting,
            color3,
            moons,
            swing,
            jetpack,
            demon_info,
            level_info,
            platformer_info
            FROM users
            WHERE username ILIKE '%' || $1 || '%'
            "#,
            search
        )
        .fetch_all(pool)
        .await?;

        Ok(users)
    }

    pub async fn id_from_username(pool: &PgPool, username: &str) -> Result<i32> {
        let user_id = sqlx::query_scalar!("SELECT id FROM users WHERE username = $1", username)
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
            "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1) AS \"exists!\"",
            username
        )
        .fetch_one(pool)
        .await?;

        Ok(exists)
    }

    pub async fn is_email_taken(pool: &PgPool, email: &str) -> Result<bool> {
        let exists: bool = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1) AS \"exists!\"",
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
