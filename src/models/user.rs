use anyhow::Result;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use gd_response_derive::GDResponse;
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use sqlx::{FromRow, PgPool};

#[derive(Serialize, Deserialize, Debug, FromRow, GDResponse)]
pub struct User {
    #[response(1)]
    username: String,

    #[response(2)]
    id: i32,

    #[response(3)]
    stars: i32,

    #[response(4)]
    demons: i32,

    #[response(8)]
    creator_points: i32,

    #[response(10)]
    color1: i16,

    #[response(11)]
    color2: i16,

    #[response(13)]
    coins: i32,

    #[response(16)]
    account_id: i32,

    #[response(17)]
    user_coins: i32,

    #[response(18)]
    message_setting: i16,

    #[response(19)]
    friend_setting: i16,

    #[response(20)]
    youtube: String,

    #[response(21)]
    icon: i16,

    #[response(22)]
    ship: i16,

    #[response(23)]
    ball: i16,

    #[response(24)]
    ufo: i16,

    #[response(25)]
    wave: i16,

    #[response(26)]
    robot: i16,

    #[response(28)]
    glow: i16,

    #[response(29)]
    is_activated: i16,

    #[response(30)]
    rank: i32,

    #[response(31)]
    friend_state: i16,

    #[response(43)]
    spider: i16,

    #[response(44)]
    twitter: String,

    #[response(45)]
    twitch: String,

    #[response(46)]
    diamonds: i32,

    #[response(48)]
    explosion: i16,

    #[response(49)]
    mod_level: i16,

    #[response(50)]
    comment_setting: i16,

    #[response(51)]
    color3: i16,

    #[response(52)]
    moons: i32,

    #[response(53)]
    swing: i16,

    #[response(54)]
    jetpack: i16,

    #[response(55)]
    demon_info: String,

    #[response(56)]
    level_info: String,

    #[response(57)]
    platformer_info: String,
}

impl User {
    pub fn generate_gjp2(password: &str) -> String {
        let salt = "mI29fmAnxgTs";
        let salted_password = password.to_owned() + salt;

        let mut hasher = Sha1::new();
        hasher.update(salted_password);
        let hashed = hasher.finalize();

        format!("{:x}", hashed)
    }

    /// Hashes a GJP2 with Argon2
    pub fn hash_gjp2(gjp2: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(gjp2.as_bytes(), &salt)
            .expect("Failed to hash")
            .to_string();
        Ok(password_hash)
    }

    pub async fn verify_gjp2(pool: &PgPool, user_id: i32, gjp2: &str) -> Result<bool> {
        let argon2 = Argon2::default();
        let hash: String = sqlx::query_scalar!("SELECT hash FROM users WHERE id = $1", user_id)
            .fetch_one(pool)
            .await?;

        let hash = PasswordHash::new(&hash).expect("Failed to parse hash");

        Ok(argon2.verify_password(gjp2.as_bytes(), &hash).is_ok())
    }

    pub async fn register(
        pool: &PgPool,
        username: &str,
        password: &str,
        email: &str,
    ) -> Result<()> {
        let gjp2 = User::generate_gjp2(password);
        let hash = User::hash_gjp2(&gjp2)?;

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

    pub async fn get_user_id(pool: &PgPool, username: &str) -> Result<i32> {
        let user_id: i32 =
            sqlx::query_scalar!("SELECT id FROM users WHERE username = $1", username)
                .fetch_one(pool)
                .await?;

        Ok(user_id)
    }

    pub async fn save_data(pool: &PgPool, user_id: i32, save_data: &str) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE users
            SET save_data = $1
            WHERE id = $2
            "#,
            save_data,
            user_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn load_data(pool: &PgPool, user_id: i32) -> Result<String> {
        let save_data = sqlx::query_scalar!("SELECT save_data FROM users WHERE id = $1", user_id)
            .fetch_one(pool)
            .await?;

        Ok(save_data)
    }

    pub async fn to_gd(pool: &PgPool, user_id: i32) -> Result<String> {
        let user: Self = sqlx::query_as!(
            Self,
            r#"
            SELECT
                username,
                id,
                stars,
                demons,
                creator_points,
                color1,
                color2,
                coins,
                account_id,
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
                friend_state,
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
            WHERE id = $1
            "#,
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(user.to_gd_response(":"))
    }
}
